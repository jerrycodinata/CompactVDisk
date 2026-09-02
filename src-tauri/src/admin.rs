use crate::models::ToolAvailability;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn create_command(program: &str) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(program);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    cmd
}

pub fn relaunch_as_administrator() -> bool {
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy().to_string();
            let mut cmd = create_command("powershell.exe");
            cmd.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!("Start-Process -FilePath \"{}\" -Verb RunAs", exe_str),
            ]);
            if let Ok(mut child) = cmd.spawn() {
                let status = child.wait();
                return status.map(|s| s.success()).unwrap_or(false);
            }
        }
    }
    false
}

pub fn is_tool_in_path(tool_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    let check_cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let check_cmd = "which";

    create_command(check_cmd)
        .arg(tool_name)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

pub fn check_admin_privileges() -> bool {
    #[cfg(target_os = "windows")]
    {
        create_command("net")
            .arg("session")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Check uid via /usr/bin/id command on Linux/macOS
        create_command("id")
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
