use crate::disk_inspector::{detect_format, format_size};
use crate::models::{CompactLogEvent, CompactionResult, DiskFormat};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;
use tauri::{AppHandle, Emitter};

pub fn generate_diskpart_script(vdisk_path: &str) -> String {
    format!(
        "select vdisk file=\"{}\"\ncompact vdisk\ndetach vdisk\n",
        vdisk_path
    )
}

pub fn build_compaction_commands(path: &str) -> Result<(String, Vec<Vec<String>>, Option<String>), String> {
    let format = detect_format(path);
    match format {
        DiskFormat::Vhdx => {
            let script_content = generate_diskpart_script(path);
            Ok((
                "diskpart".to_string(),
                vec![
                    vec!["wsl.exe".to_string(), "--shutdown".to_string()],
                    vec!["diskpart.exe".to_string(), "/s".to_string(), "{SCRIPT_PATH}".to_string()],
                ],
                Some(script_content),
            ))
        }
        DiskFormat::Vmdk => {
            Ok((
                "vmdk".to_string(),
                vec![
                    vec!["vmware-vdiskmanager".to_string(), "-k".to_string(), path.to_string()],
                ],
                None,
            ))
        }
        DiskFormat::Vdi => {
            Ok((
                "vdi".to_string(),
                vec![
                    vec!["vboxmanage".to_string(), "modifymedium".to_string(), "disk".to_string(), path.to_string(), "--compact".to_string()],
                ],
                None,
            ))
        }
        DiskFormat::Unknown => Err(format!("Unsupported disk format for path: {}", path)),
    }
}

pub async fn execute_compaction(
    app: AppHandle,
    disk_id: String,
    path_str: String,
) -> Result<CompactionResult, String> {
    let path = Path::new(&path_str);
    if !path.exists() {
        return Err(format!("Disk file not found: {}", path_str));
    }

    let start_time = Instant::now();
    let initial_metadata = fs::metadata(path).map_err(|e| format!("Failed to read file size: {}", e))?;
    let initial_size_bytes = initial_metadata.len();

    let emit_log = |app_handle: &AppHandle, line: String, is_error: bool| {
        let _ = app_handle.emit(
            "compact-log",
            CompactLogEvent {
                disk_id: disk_id.clone(),
                line,
                is_error,
            },
        );
    };

    emit_log(&app, format!("Starting compaction process for {}", path_str), false);
    emit_log(&app, format!("Initial disk size: {}", format_size(initial_size_bytes)), false);

    let format = detect_format(&path_str);

    let mut success = true;
    let mut message = String::from("Compaction completed successfully.");

    match format {
        DiskFormat::Vhdx => {
            emit_log(&app, "Executing: wsl.exe --shutdown".to_string(), false);
            let shutdown_res = Command::new("wsl.exe").arg("--shutdown").output();
            match shutdown_res {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if !stdout.trim().is_empty() {
                        emit_log(&app, stdout.to_string(), false);
                    }
                }
                Err(e) => {
                    emit_log(&app, format!("Notice: wsl.exe --shutdown returned: {}", e), false);
                }
            }

            let script_content = generate_diskpart_script(&path_str);
            let temp_dir = std::env::temp_dir();
            let script_path = temp_dir.join("compact_vdisk_script.txt");

            if let Err(e) = fs::write(&script_path, &script_content) {
                let err_msg = format!("Failed to write diskpart script: {}", e);
                emit_log(&app, err_msg.clone(), true);
                return Err(err_msg);
            }

            emit_log(&app, format!("Executing: diskpart.exe /s {}", script_path.display()), false);

            let mut child = Command::new("diskpart.exe")
                .arg("/s")
                .arg(&script_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to execute diskpart.exe: {}", e))?;

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    emit_log(&app, line, false);
                }
            }

            let status = child.wait().map_err(|e| format!("diskpart.exe execution failed: {}", e))?;
            let _ = fs::remove_file(&script_path);

            if !status.success() {
                success = false;
                message = "diskpart.exe returned a non-zero exit code.".to_string();
            }
        }
        DiskFormat::Vmdk => {
            emit_log(&app, format!("Executing: vmware-vdiskmanager -k {}", path_str), false);
            let mut child = Command::new("vmware-vdiskmanager")
                .arg("-k")
                .arg(&path_str)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(mut proc) => {
                    if let Some(stdout) = proc.stdout.take() {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines().flatten() {
                            emit_log(&app, line, false);
                        }
                    }
                    let status = proc.wait().map_err(|e| format!("vmware-vdiskmanager failed: {}", e))?;
                    if !status.success() {
                        success = false;
                        message = "vmware-vdiskmanager returned an error.".to_string();
                    }
                }
                Err(_) => {
                    emit_log(&app, "vmware-vdiskmanager not found. Attempting qemu-img convert...".to_string(), false);
                    let temp_vmdk = format!("{}.compact.tmp", path_str);
                    let qemu_res = Command::new("qemu-img")
                        .args(["convert", "-O", "vmdk", "-c", &path_str, &temp_vmdk])
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn();

                    match qemu_res {
                        Ok(mut proc) => {
                            if let Some(stdout) = proc.stdout.take() {
                                let reader = BufReader::new(stdout);
                                for line in reader.lines().flatten() {
                                    emit_log(&app, line, false);
                                }
                            }
                            let status = proc.wait().map_err(|e| format!("qemu-img failed: {}", e))?;
                            if status.success() {
                                let _ = fs::rename(&temp_vmdk, &path_str);
                            } else {
                                let _ = fs::remove_file(&temp_vmdk);
                                success = false;
                                message = "qemu-img conversion failed.".to_string();
                            }
                        }
                        Err(e) => {
                            let err_msg = format!("Neither vmware-vdiskmanager nor qemu-img are available in PATH: {}", e);
                            emit_log(&app, err_msg.clone(), true);
                            return Err(err_msg);
                        }
                    }
                }
            }
        }
        DiskFormat::Vdi => {
            emit_log(&app, format!("Executing: vboxmanage modifymedium disk {} --compact", path_str), false);
            let child = Command::new("vboxmanage")
                .args(["modifymedium", "disk", &path_str, "--compact"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(mut proc) => {
                    if let Some(stdout) = proc.stdout.take() {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines().flatten() {
                            emit_log(&app, line, false);
                        }
                    }
                    let status = proc.wait().map_err(|e| format!("vboxmanage failed: {}", e))?;
                    if !status.success() {
                        success = false;
                        message = "vboxmanage returned an error status.".to_string();
                    }
                }
                Err(e) => {
                    let err_msg = format!("VBoxManage executable not found in PATH: {}", e);
                    emit_log(&app, err_msg.clone(), true);
                    return Err(err_msg);
                }
            }
        }
        DiskFormat::Unknown => {
            return Err("Unknown disk format cannot be compacted.".to_string());
        }
    }

    let elapsed_seconds = start_time.elapsed().as_secs_f64();
    let new_metadata = fs::metadata(path).map_err(|e| format!("Failed to read new file size: {}", e))?;
    let new_size_bytes = new_metadata.len();
    let reclaimed_bytes = if initial_size_bytes > new_size_bytes {
        initial_size_bytes - new_size_bytes
    } else {
        0
    };

    let result = CompactionResult {
        initial_size_bytes,
        new_size_bytes,
        reclaimed_bytes,
        initial_size_formatted: format_size(initial_size_bytes),
        new_size_formatted: format_size(new_size_bytes),
        reclaimed_formatted: format_size(reclaimed_bytes),
        elapsed_seconds,
        success,
        message,
    };

    emit_log(&app, format!("Compaction finished in {:.2}s. Reclaimed: {}", elapsed_seconds, result.reclaimed_formatted), false);

    Ok(result)
}
