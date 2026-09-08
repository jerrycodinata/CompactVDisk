#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use compact_vdisk_lib::admin::{check_admin_privileges, get_tool_availability, relaunch_as_administrator};
use compact_vdisk_lib::compactor::execute_compaction;
use compact_vdisk_lib::disk_inspector::get_disk_info;
use compact_vdisk_lib::models::{CompactionResult, DiskInfo, DiskType, ToolAvailability};
use compact_vdisk_lib::wsl_discovery::discover_disks as run_wsl_discovery;
use tauri::AppHandle;

#[tauri::command]
fn check_admin() -> bool {
    check_admin_privileges()
}

#[tauri::command]
fn relaunch_as_admin(app: AppHandle) -> bool {
    if relaunch_as_administrator() {
        app.exit(0);
        true
    } else {
        false
    }
}

#[tauri::command]
fn check_tools() -> ToolAvailability {
    get_tool_availability()
}

#[tauri::command]
fn discover_disks() -> Vec<DiskInfo> {
    run_wsl_discovery()
}

#[tauri::command]
fn inspect_disk(path: String) -> Result<DiskInfo, String> {
    get_disk_info(&path, None, DiskType::Custom)
}

#[tauri::command]
async fn compact_disk(app: AppHandle, disk_id: String, path: String) -> Result<CompactionResult, String> {
    execute_compaction(app, disk_id, path).await
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Prevent WebView2 initialization failures when running elevated or portable.
        // Ensure WEBVIEW2_USER_DATA_FOLDER is configured to an accessible location if not set.
        if std::env::var_os("WEBVIEW2_USER_DATA_FOLDER").is_none() {
            let candidates = [
                std::env::var("LOCALAPPDATA").ok().map(std::path::PathBuf::from),
                std::env::var("TEMP").ok().map(std::path::PathBuf::from),
                std::env::var("PROGRAMDATA").ok().map(std::path::PathBuf::from),
                Some(std::env::temp_dir()),
            ];

            for candidate in candidates.into_iter().flatten() {
                let udf = candidate.join("CompactVdisk").join("EBWebView");
                if std::fs::create_dir_all(&udf).is_ok() {
                    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", udf.to_string_lossy().to_string());
                    break;
                }
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_admin,
            relaunch_as_admin,
            check_tools,
            discover_disks,
            inspect_disk,
            compact_disk
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
