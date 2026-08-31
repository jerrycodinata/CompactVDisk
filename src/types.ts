export type DiskFormat = 'Vhdx' | 'Vmdk' | 'Vdi' | 'Unknown';
export type DiskType = 'Wsl' | 'Docker' | 'Custom';

export interface DiskInfo {
  id: String;
  name: string;
  path: string;
  format: DiskFormat;
  size_bytes: number;
  size_formatted: string;
  status: string;
  disk_type: DiskType;
}

export interface ToolAvailability {
  qemu_img: boolean;
  vboxmanage: boolean;
  vmware_vdiskmanager: boolean;
  wsl: boolean;
  diskpart: boolean;
}

export interface CompactionResult {
  initial_size_bytes: number;
  new_size_bytes: number;
  reclaimed_bytes: number;
  initial_size_formatted: string;
  new_size_formatted: string;
  reclaimed_formatted: string;
  elapsed_seconds: number;
  success: boolean;
  message: string;
}

export interface CompactLogEvent {
  disk_id: string;
  line: string;
  is_error: boolean;
}
