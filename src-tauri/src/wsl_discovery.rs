use crate::admin::create_command;
use crate::disk_inspector::{clean_disk_name, detect_format, format_size};
use crate::models::{DiskFormat, DiskInfo, DiskType};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
    let output = create_command("wsl.exe")
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
    let name_lower = distro_name.to_lowercase();
    if name_lower.contains("docker") {
        if name_lower.contains("data") {
            let data_path = local_app_data.join("Docker").join("wsl").join("data").join("ext4.vhdx");
            if data_path.exists() {
                return Some(data_path);
            }
        } else {
            let distro_path = local_app_data.join("Docker").join("wsl").join("distro").join("ext4.vhdx");
            if distro_path.exists() {
                return Some(distro_path);
            }
            let main_path = local_app_data.join("Docker").join("wsl").join("main").join("ext4.vhdx");
            if main_path.exists() {
                return Some(main_path);
            }
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

fn normalize_path_key(p: &Path) -> String {
    if let Ok(canon) = p.canonicalize() {
        canon.to_string_lossy().to_string().to_lowercase().trim_start_matches(r"\\?\").to_string()
    } else {
        p.to_string_lossy().to_string().to_lowercase()
    }
}

fn parse_docker_settings() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let candidates = vec![
        env::var("APPDATA").ok().map(PathBuf::from),
        env::var("LOCALAPPDATA").ok().map(PathBuf::from),
    ];

    for base in candidates.into_iter().flatten() {
        let settings_file = base.join("Docker").join("settings.json");
        if settings_file.exists() {
            if let Ok(content) = fs::read_to_string(&settings_file) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    for key in &["dataFolder", "diskPath", "dataDiskPath", "wslEnginePath"] {
                        if let Some(val) = json.get(key).and_then(|v| v.as_str()) {
                            let path = PathBuf::from(val);
                            if path.is_file() {
                                paths.push(path);
                            } else if path.is_dir() {
                                for name in &["ext4.vhdx", "disk.vhdx", "DockerDesktop.vhdx"] {
                                    let cand = path.join(name);
                                    if cand.exists() {
                                        paths.push(cand);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    paths
}

fn scan_directory_for_virtual_disks(dir: &Path, max_depth: usize, current_depth: usize, found_files: &mut Vec<PathBuf>) {
    if current_depth > max_depth || !dir.exists() || !dir.is_dir() {
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let fmt = detect_format(&path.to_string_lossy());
                if fmt != DiskFormat::Unknown {
                    found_files.push(path);
                }
            } else if path.is_dir() && current_depth < max_depth {
                scan_directory_for_virtual_disks(&path, max_depth, current_depth + 1, found_files);
            }
        }
    }
}

pub fn discover_disks() -> Vec<DiskInfo> {
    let mut disks: Vec<DiskInfo> = Vec::new();
    let mut seen_keys: Vec<String> = Vec::new();

    let candidate_dirs = get_candidate_local_appdata_dirs();
    let distros = get_wsl_distros();

    let add_disk = |disks: &mut Vec<DiskInfo>, seen_keys: &mut Vec<String>, path: PathBuf, raw_name: String, status: String, default_type: DiskType| {
        if !path.exists() {
            return;
        }
        let key = normalize_path_key(&path);
        if seen_keys.contains(&key) {
            return;
        }

        if let Ok(metadata) = fs::metadata(&path) {
            let path_str = path.to_string_lossy().to_string();
            let is_docker = raw_name.to_lowercase().contains("docker") || path_str.to_lowercase().contains("docker");
            let disk_type = if is_docker {
                DiskType::Docker
            } else {
                default_type
            };
            let cleaned_name = clean_disk_name(&raw_name, &path_str);
            let format = detect_format(&path_str);

            seen_keys.push(key);
            disks.push(DiskInfo {
                id: path_str.clone(),
                name: cleaned_name,
                path: path_str,
                format,
                size_bytes: metadata.len(),
                size_formatted: format_size(metadata.len()),
                status,
                disk_type,
            });
        }
    };

    for (distro_name, state) in distros {
        let mut found_path = None;
        for lad in &candidate_dirs {
            if let Some(vhdx_path) = find_vhdx_for_wsl_in_dir(&distro_name, lad) {
                found_path = Some(vhdx_path);
                break;
            }
        }

        if let Some(vhdx_path) = found_path {
            let is_docker = distro_name.to_lowercase().contains("docker");
            add_disk(
                &mut disks,
                &mut seen_keys,
                vhdx_path,
                distro_name,
                state,
                if is_docker { DiskType::Docker } else { DiskType::Wsl },
            );
        }
    }

    for docker_file in parse_docker_settings() {
        add_disk(
            &mut disks,
            &mut seen_keys,
            docker_file,
            "Docker Desktop".to_string(),
            "Ready".to_string(),
            DiskType::Docker,
        );
    }

    for lad in &candidate_dirs {
        let docker_wsl_dir = lad.join("Docker").join("wsl");
        if docker_wsl_dir.exists() {
            for sub in &["data", "distro", "main"] {
                let cand = docker_wsl_dir.join(sub).join("ext4.vhdx");
                if cand.exists() {
                    let label = format!("Docker Desktop {}", sub);
                    add_disk(&mut disks, &mut seen_keys, cand, label, "Ready".to_string(), DiskType::Docker);
                }
            }
        }

        let wsl_dir = lad.join("wsl");
        if wsl_dir.exists() {
            if let Ok(entries) = fs::read_dir(&wsl_dir) {
                for entry in entries.flatten() {
                    let vhdx_file = entry.path().join("ext4.vhdx");
                    if vhdx_file.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        add_disk(&mut disks, &mut seen_keys, vhdx_file, name, "Ready".to_string(), DiskType::Wsl);
                    }
                }
            }
        }

        let packages_dir = lad.join("Packages");
        if packages_dir.exists() {
            if let Ok(entries) = fs::read_dir(&packages_dir) {
                for entry in entries.flatten() {
                    let vhdx_file = entry.path().join("LocalState").join("ext4.vhdx");
                    if vhdx_file.exists() {
                        let pkg_name = entry.file_name().to_string_lossy().to_string();
                        add_disk(&mut disks, &mut seen_keys, vhdx_file, pkg_name, "Ready".to_string(), DiskType::Wsl);
                    }
                }
            }
        }
    }

    let mut custom_wsl_dirs = vec![
        PathBuf::from("C:\\wsl"),
        PathBuf::from("D:\\wsl"),
        PathBuf::from("C:\\WSL"),
        PathBuf::from("D:\\WSL"),
    ];
    if let Ok(up) = env::var("USERPROFILE") {
        custom_wsl_dirs.push(PathBuf::from(up).join("wsl"));
    }

    for dir in custom_wsl_dirs {
        if dir.exists() && dir.is_dir() {
            let mut found = Vec::new();
            scan_directory_for_virtual_disks(&dir, 2, 0, &mut found);
            for f in found {
                let name = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                add_disk(&mut disks, &mut seen_keys, f, name, "Ready".to_string(), DiskType::Wsl);
            }
        }
    }

    let hyperv_dirs = vec![
        PathBuf::from("C:\\Users\\Public\\Documents\\Hyper-V\\Virtual Hard Disks"),
        PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Hyper-V\\Virtual Hard Disks"),
    ];
    for dir in hyperv_dirs {
        if dir.exists() && dir.is_dir() {
            let mut found = Vec::new();
            scan_directory_for_virtual_disks(&dir, 2, 0, &mut found);
            for f in found {
                let name = f.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                add_disk(&mut disks, &mut seen_keys, f, name, "Ready".to_string(), DiskType::Custom);
            }
        }
    }

    if let Ok(up) = env::var("USERPROFILE") {
        let up_path = PathBuf::from(up);
        let vbox_dir = up_path.join("VirtualBox VMs");
        if vbox_dir.exists() {
            let mut found = Vec::new();
            scan_directory_for_virtual_disks(&vbox_dir, 2, 0, &mut found);
            for f in found {
                let name = f.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                add_disk(&mut disks, &mut seen_keys, f, name, "Ready".to_string(), DiskType::Custom);
            }
        }

        let vmware_dir = up_path.join("Documents").join("Virtual Machines");
        if vmware_dir.exists() {
            let mut found = Vec::new();
            scan_directory_for_virtual_disks(&vmware_dir, 2, 0, &mut found);
            for f in found {
                let name = f.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                add_disk(&mut disks, &mut seen_keys, f, name, "Ready".to_string(), DiskType::Custom);
            }
        }
    }

    if let Ok(pd) = env::var("PROGRAMDATA") {
        let multipass_dir = PathBuf::from(pd).join("Multipass").join("data").join("vault").join("instances");
        if multipass_dir.exists() {
            let mut found = Vec::new();
            scan_directory_for_virtual_disks(&multipass_dir, 2, 0, &mut found);
            for f in found {
                let name = f.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                add_disk(&mut disks, &mut seen_keys, f, format!("Multipass {}", name), "Ready".to_string(), DiskType::Custom);
            }
        }
    }

    disks
}
