use crate::models::ToolAvailability;
use std::process::Command;

pub fn is_tool_in_path(tool_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    let check_cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let check_cmd = "which";

    Command::new(check_cmd)
        .arg(tool_name)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

pub fn check_admin_privileges() -> bool {
    #[cfg(target_os = "windows")]
    {
        Command::new("net")
            .arg("session")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Check uid via /usr/bin/id command on Linux/macOS
        Command::new("id")
            .arg("-u")
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).trim() == "0")
            .unwrap_or(false)
    }
}

pub fn get_tool_availability() -> ToolAvailability {
    ToolAvailability {
        qemu_img: is_tool_in_path("qemu-img"),
        vboxmanage: is_tool_in_path("vboxmanage") || is_tool_in_path("VBoxManage"),
        vmware_vdiskmanager: is_tool_in_path("vmware-vdiskmanager"),
        wsl: is_tool_in_path("wsl") || is_tool_in_path("wsl.exe"),
        diskpart: is_tool_in_path("diskpart") || is_tool_in_path("diskpart.exe"),
    }
}
