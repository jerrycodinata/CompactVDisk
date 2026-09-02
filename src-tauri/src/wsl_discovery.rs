use crate::disk_inspector::format_size;
use crate::models::{DiskFormat, DiskInfo, DiskType};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn parse_wsl_list_output(output: &str) -> Vec<(String, String)> {
    let mut distros = Vec::new();
    let lines: Vec<&str> = output.lines().collect();

    for line in lines {
        let clean = line.replace('\0', "");
        let clean = clean.trim();
        if clean.is_empty() || clean.starts_with("NAME") || clean.starts_with("---") {
            continue;
        }

        let line_without_star = clean.trim_start_matches('*').trim();
        let parts: Vec<&str> = line_without_star.split_whitespace().collect();

        if !parts.is_empty() {
            let name = parts[0].to_string();
            let state = if parts.len() > 1 { parts[1].to_string() } else { "Unknown".to_string() };
            distros.push((name, state));
        }
    }

    distros
}

pub fn get_wsl_distros() -> Vec<(String, String)> {
    let output = Command::new("wsl.exe")
        .args(["-l", "-v"])
        .output();

    match output {
        Ok(out) => {
            if out.stdout.contains(&0) {
                let utf16_str = parse_utf16le(&out.stdout);
                parse_wsl_list_output(&utf16_str)
            } else {
                let stdout_str = String::from_utf8_lossy(&out.stdout).to_string();
                parse_wsl_list_output(&stdout_str)
            }
        }
        Err(_) => Vec::new(),
    }
}

fn parse_utf16le(bytes: &[u8]) -> String {
    let u16_vec: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&u16_vec)
}

pub fn find_vhdx_for_wsl_in_dir(distro_name: &str, local_app_data: &Path) -> Option<PathBuf> {
    if distro_name.contains("docker") {
        let docker_path = local_app_data.join("Docker").join("wsl").join("data").join("ext4.vhdx");
        if docker_path.exists() {
            return Some(docker_path);
        }
        let docker_distro_path = local_app_data.join("Docker").join("wsl").join(distro_name).join("ext4.vhdx");
        if docker_distro_path.exists() {
            return Some(docker_distro_path);
        }
    }

    let wsl_root = local_app_data.join("wsl");
    if wsl_root.exists() {
        let candidate = wsl_root.join(distro_name).join("ext4.vhdx");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    let packages_dir = local_app_data.join("Packages");
    if packages_dir.exists() {
        if let Ok(entries) = fs::read_dir(&packages_dir) {
            let name_lower = distro_name.to_lowercase();
            for entry in entries.flatten() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                if folder_name.to_lowercase().contains(&name_lower) || (name_lower.contains("ubuntu") && folder_name.to_lowercase().contains("ubuntu")) {
                    let vhdx_candidate = entry.path().join("LocalState").join("ext4.vhdx");
                    if vhdx_candidate.exists() {
                        return Some(vhdx_candidate);
                    }
                }
            }
        }
    }

    None
}

pub fn get_candidate_local_appdata_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(lad) = env::var("LOCALAPPDATA") {
        let p = PathBuf::from(lad);
        if p.exists() && !dirs.contains(&p) {
            dirs.push(p);
        }
    }

    if let Ok(up) = env::var("USERPROFILE") {
        let p = PathBuf::from(up).join("AppData").join("Local");
        if p.exists() && !dirs.contains(&p) {
            dirs.push(p);
        }
    }

    let users_dir = Path::new("C:\\Users");
    if users_dir.exists() {
        if let Ok(entries) = fs::read_dir(users_dir) {
            for entry in entries.flatten() {
                let user_folder = entry.path();
                if user_folder.is_dir() {
                    let lad = user_folder.join("AppData").join("Local");
                    if lad.exists() && !dirs.contains(&lad) {
                        dirs.push(lad);
                    }
                }
            }
        }
    }

    dirs
}

pub fn find_vhdx_for_wsl(distro_name: &str, _local_app_data: &Path) -> Option<PathBuf> {
    let search_dirs = get_candidate_local_appdata_dirs();
    for dir in search_dirs {
        if let Some(vhdx) = find_vhdx_for_wsl_in_dir(distro_name, &dir) {
            return Some(vhdx);
        }
    }
    None
}

pub fn discover_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();

    let candidate_dirs = get_candidate_local_appdata_dirs();
    let distros = get_wsl_distros();

    for (distro_name, state) in distros {
        let mut found_path = None;
        for lad in &candidate_dirs {
            if let Some(vhdx_path) = find_vhdx_for_wsl_in_dir(&distro_name, lad) {
                found_path = Some(vhdx_path);
                break;
            }
        }

        if let Some(vhdx_path) = found_path {
            if let Ok(metadata) = fs::metadata(&vhdx_path) {
                let path_str = vhdx_path.to_string_lossy().to_string();
                let is_docker = distro_name.contains("docker");
                if !disks.iter().any(|d: &DiskInfo| d.path == path_str) {
                    disks.push(DiskInfo {
                        id: path_str.clone(),
                        name: distro_name.clone(),
                        path: path_str,
                        format: DiskFormat::Vhdx,
                        size_bytes: metadata.len(),
                        size_formatted: format_size(metadata.len()),
                        status: state,
                        disk_type: if is_docker { DiskType::Docker } else { DiskType::Wsl },
                    });
                }
            }
        }
    }

    for lad in &candidate_dirs {
        let docker_default = lad.join("Docker").join("wsl").join("data").join("ext4.vhdx");
        if docker_default.exists() {
            let path_str = docker_default.to_string_lossy().to_string();
            if !disks.iter().any(|d| d.path == path_str) {
                if let Ok(metadata) = fs::metadata(&docker_default) {
                    disks.push(DiskInfo {
                        id: path_str.clone(),
                        name: "Docker Desktop WSL Data".to_string(),
                        path: path_str,
                        format: DiskFormat::Vhdx,
                        size_bytes: metadata.len(),
                        size_formatted: format_size(metadata.len()),
                        status: "Stopped/Running".to_string(),
                        disk_type: DiskType::Docker,
                    });
                }
            }
        }

        let wsl_dir = lad.join("wsl");
        if wsl_dir.exists() {
            if let Ok(distro_entries) = fs::read_dir(&wsl_dir) {
                for entry in distro_entries.flatten() {
                    let vhdx_file = entry.path().join("ext4.vhdx");
                    if vhdx_file.exists() {
                        let path_str = vhdx_file.to_string_lossy().to_string();
                        if !disks.iter().any(|d| d.path == path_str) {
                            if let Ok(metadata) = fs::metadata(&vhdx_file) {
                                let folder_name = entry.file_name().to_string_lossy().to_string();
                                let is_docker = folder_name.contains("docker");
                                disks.push(DiskInfo {
                                    id: path_str.clone(),
                                    name: folder_name,
                                    path: path_str,
                                    format: DiskFormat::Vhdx,
                                    size_bytes: metadata.len(),
                                    size_formatted: format_size(metadata.len()),
                                    status: "Unknown".to_string(),
                                    disk_type: if is_docker { DiskType::Docker } else { DiskType::Wsl },
                                });
                            }
                        }
                    }
                }
            }
        }

        let packages_dir = lad.join("Packages");
        if packages_dir.exists() {
            if let Ok(pkg_entries) = fs::read_dir(&packages_dir) {
                for entry in pkg_entries.flatten() {
                    let vhdx_file = entry.path().join("LocalState").join("ext4.vhdx");
                    if vhdx_file.exists() {
                        let path_str = vhdx_file.to_string_lossy().to_string();
                        if !disks.iter().any(|d| d.path == path_str) {
                            if let Ok(metadata) = fs::metadata(&vhdx_file) {
                                let pkg_name = entry.file_name().to_string_lossy().to_string();
                                disks.push(DiskInfo {
                                    id: path_str.clone(),
                                    name: pkg_name,
                                    path: path_str,
                                    format: DiskFormat::Vhdx,
                                    size_bytes: metadata.len(),
                                    size_formatted: format_size(metadata.len()),
                                    status: "Unknown".to_string(),
                                    disk_type: DiskType::Wsl,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    disks
}
