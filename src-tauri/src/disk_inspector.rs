use crate::models::{DiskFormat, DiskInfo, DiskType};
use std::fs;
use std::path::Path;

pub fn format_size(size_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size_bytes >= TB {
        format!("{:.2} TB", size_bytes as f64 / TB as f64)
    } else if size_bytes >= GB {
        format!("{:.2} GB", size_bytes as f64 / GB as f64)
    } else if size_bytes >= MB {
        format!("{:.2} MB", size_bytes as f64 / MB as f64)
    } else if size_bytes >= KB {
        format!("{:.2} KB", size_bytes as f64 / KB as f64)
    } else {
        format!("{} B", size_bytes)
    }
}

pub fn detect_format(path: &str) -> DiskFormat {
    let path_lower = path.to_lowercase();
    if path_lower.ends_with(".vhdx") {
        DiskFormat::Vhdx
    } else if path_lower.ends_with(".vmdk") {
        DiskFormat::Vmdk
    } else if path_lower.ends_with(".vdi") {
        DiskFormat::Vdi
    } else {
        DiskFormat::Unknown
    }
}

pub fn clean_disk_name(raw_name: &str, path_str: &str) -> String {
    let lower_raw = raw_name.to_lowercase();
    let lower_path = path_str.to_lowercase();

    if lower_raw.contains("docker") || lower_path.contains("docker") {
        if lower_raw.contains("data") || lower_path.contains("wsl\\data") || lower_path.contains("wsl/data") {
            return "Docker Desktop Data".to_string();
        }
        if lower_raw.contains("distro") || lower_path.contains("wsl\\distro") || lower_path.contains("wsl/distro") {
            return "Docker Desktop Engine".to_string();
        }
        return "Docker Desktop".to_string();
    }

    if lower_raw.contains("ubuntu") || lower_path.contains("ubuntu") {
        if lower_raw.contains("24.04") || lower_path.contains("24.04") {
            return "Ubuntu 24.04 LTS".to_string();
        }
        if lower_raw.contains("22.04") || lower_path.contains("22.04") {
            return "Ubuntu 22.04 LTS".to_string();
        }
        if lower_raw.contains("20.04") || lower_path.contains("20.04") {
            return "Ubuntu 20.04 LTS".to_string();
        }
        if lower_raw.contains("18.04") || lower_path.contains("18.04") {
            return "Ubuntu 18.04 LTS".to_string();
        }
        return "Ubuntu".to_string();
    }

    if lower_raw.contains("debian") || lower_path.contains("debian") {
        return "Debian GNU/Linux".to_string();
    }

    if lower_raw.contains("kali") || lower_path.contains("kali") {
        return "Kali Linux".to_string();
    }

    if lower_raw.contains("alpine") || lower_path.contains("alpine") {
        return "Alpine Linux".to_string();
    }

    if lower_raw.contains("arch") || lower_path.contains("arch") {
        return "Arch Linux".to_string();
    }

    if lower_raw.contains("rhel") || lower_path.contains("rhel") || lower_raw.contains("redhat") {
        return "Red Hat Enterprise Linux".to_string();
    }

    if lower_raw.contains("fedora") || lower_path.contains("fedora") {
        return "Fedora".to_string();
    }

    if lower_raw.contains("opensuse") || lower_path.contains("suse") {
        return "openSUSE".to_string();
    }

    if raw_name.contains('.') && raw_name.contains('_') {
        let parts: Vec<&str> = raw_name.split('_').collect();
        if let Some(pkg) = parts.first() {
            let subparts: Vec<&str> = pkg.split('.').collect();
            if subparts.len() > 1 {
                return subparts[1..].join(" ");
            }
        }
    }

    if raw_name.eq_ignore_ascii_case("ext4.vhdx")
        || raw_name.eq_ignore_ascii_case("disk.vhdx")
        || (raw_name.starts_with('{') && raw_name.ends_with('}'))
    {
        let p = Path::new(path_str);
        if let Some(parent) = p.parent() {
            if let Some(dir_name) = parent.file_name() {
                let name_str = dir_name.to_string_lossy();
                if !name_str.is_empty() && name_str != "LocalState" && name_str != "wsl" {
                    return clean_disk_name(&name_str, path_str);
                }
            }
            if let Some(grandparent) = parent.parent() {
                if let Some(gp_name) = grandparent.file_name() {
                    let gp_str = gp_name.to_string_lossy();
                    if !gp_str.is_empty() {
                        return clean_disk_name(&gp_str, path_str);
                    }
                }
            }
        }
    }

    raw_name.to_string()
}

pub fn get_disk_info(path_str: &str, custom_name: Option<String>, disk_type: DiskType) -> Result<DiskInfo, String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Err(format!("File does not exist: {}", path_str));
    }

    let metadata = fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let size_bytes = metadata.len();
    let format = detect_format(path_str);

    let raw_name = custom_name.unwrap_or_else(|| {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path_str.to_string())
    });

    let name = clean_disk_name(&raw_name, path_str);

    Ok(DiskInfo {
        id: path_str.to_string(),
        name,
        path: path_str.to_string(),
        format,
        size_bytes,
        size_formatted: format_size(size_bytes),
        status: "Ready".to_string(),
        disk_type,
    })
}
