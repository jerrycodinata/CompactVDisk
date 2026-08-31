use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiskFormat {
    Vhdx,
    Vmdk,
    Vdi,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiskType {
    Wsl,
    Docker,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub format: DiskFormat,
    pub size_bytes: u64,
    pub size_formatted: String,
    pub status: String,
    pub disk_type: DiskType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolAvailability {
    pub qemu_img: bool,
    pub vboxmanage: bool,
    pub vmware_vdiskmanager: bool,
    pub wsl: bool,
    pub diskpart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionResult {
    pub initial_size_bytes: u64,
    pub new_size_bytes: u64,
    pub reclaimed_bytes: u64,
    pub initial_size_formatted: String,
    pub new_size_formatted: String,
    pub reclaimed_formatted: String,
    pub elapsed_seconds: f64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactLogEvent {
    pub disk_id: String,
    pub line: String,
    pub is_error: bool,
}
