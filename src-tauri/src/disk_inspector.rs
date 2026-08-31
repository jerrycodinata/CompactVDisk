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

pub fn get_disk_info(path_str: &str, custom_name: Option<String>, disk_type: DiskType) -> Result<DiskInfo, String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Err(format!("File does not exist: {}", path_str));
    }

    let metadata = fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let size_bytes = metadata.len();
    let format = detect_format(path_str);

    let name = custom_name.unwrap_or_else(|| {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path_str.to_string())
    });

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
