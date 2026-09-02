#[cfg(test)]
mod tests {
    use crate::compactor::{build_compaction_commands, generate_diskpart_script};
    use crate::disk_inspector::{detect_format, format_size};
    use crate::models::DiskFormat;
    use crate::wsl_discovery::parse_wsl_list_output;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(2048), "2.00 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
        assert_eq!(format_size(15728640000), "14.65 GB");
        assert_eq!(format_size(1099511627776), "1.00 TB");
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("C:\\VirtualDisks\\ubuntu.VHDX"), DiskFormat::Vhdx);
        assert_eq!(detect_format("/home/user/disk.vmdk"), DiskFormat::Vmdk);
        assert_eq!(detect_format("/home/user/disk.vdi"), DiskFormat::Vdi);
        assert_eq!(detect_format("image.iso"), DiskFormat::Unknown);
    }

    #[test]
    fn test_generate_diskpart_script() {
        let path = "C:\\Users\\test\\AppData\\Local\\Docker\\wsl\\data\\ext4.vhdx";
        let script = generate_diskpart_script(path);
        assert!(script.contains(&format!("select vdisk file=\"{}\"", path)));
        assert!(script.contains("compact vdisk"));
        assert!(script.contains("detach vdisk"));
    }

    #[test]
    fn test_parse_wsl_list_output() {
        let sample_output = "  NAME                   STATE           VERSION\n* Ubuntu-22.04           Running         2\n  docker-desktop-data    Stopped         2\n";
        let parsed = parse_wsl_list_output(sample_output);

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], ("Ubuntu-22.04".to_string(), "Running".to_string()));
        assert_eq!(parsed[1], ("docker-desktop-data".to_string(), "Stopped".to_string()));
    }

    #[test]
    fn test_find_vhdx_in_dir_structure() {
        use crate::wsl_discovery::find_vhdx_for_wsl_in_dir;
        use std::fs::{create_dir_all, File};

        let temp_dir = tempfile::tempdir().unwrap();
        let lad = temp_dir.path();

        let wsl_distro_dir = lad.join("wsl").join("Ubuntu");
        create_dir_all(&wsl_distro_dir).unwrap();
        let vhdx_file = wsl_distro_dir.join("ext4.vhdx");
        File::create(&vhdx_file).unwrap();

        let found = find_vhdx_for_wsl_in_dir("Ubuntu", lad);
        assert_eq!(found, Some(vhdx_file));
    }

    #[test]
    fn test_build_compaction_commands() {
        let vhdx_cmd = build_compaction_commands("C:\\disk.vhdx").unwrap();
        assert_eq!(vhdx_cmd.0, "diskpart");
        assert!(vhdx_cmd.2.unwrap().contains("compact vdisk"));

        let vmdk_cmd = build_compaction_commands("/path/disk.vmdk").unwrap();
        assert_eq!(vmdk_cmd.0, "vmdk");

        let vdi_cmd = build_compaction_commands("/path/disk.vdi").unwrap();
        assert_eq!(vdi_cmd.0, "vdi");

        assert!(build_compaction_commands("/path/file.txt").is_err());
    }
}
