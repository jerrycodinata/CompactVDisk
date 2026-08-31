import { describe, it, expect } from 'vitest';

export function formatBytes(sizeBytes: number): string {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;
  const TB = GB * 1024;

  if (sizeBytes >= TB) {
    return `${(sizeBytes / TB).toFixed(2)} TB`;
  } else if (sizeBytes >= GB) {
    return `${(sizeBytes / GB).toFixed(2)} GB`;
  } else if (sizeBytes >= MB) {
    return `${(sizeBytes / MB).toFixed(2)} MB`;
  } else if (sizeBytes >= KB) {
    return `${(sizeBytes / KB).toFixed(2)} KB`;
  } else {
    return `${sizeBytes} B`;
  }
}

export function detectFormatFromPath(path: string): string {
  const pathLower = path.toLowerCase();
  if (pathLower.endsWith('.vhdx')) return 'Vhdx';
  if (pathLower.endsWith('.vmdk')) return 'Vmdk';
  if (pathLower.endsWith('.vdi')) return 'Vdi';
  return 'Unknown';
}

describe('Frontend Helper Utilities', () => {
  it('formats byte sizes correctly', () => {
    expect(formatBytes(500)).toBe('500 B');
    expect(formatBytes(1024)).toBe('1.00 KB');
    expect(formatBytes(1048576)).toBe('1.00 MB');
    expect(formatBytes(1073741824)).toBe('1.00 GB');
    expect(formatBytes(15258640000)).toBe('14.21 GB');
  });

  it('detects virtual disk file formats from file path', () => {
    expect(detectFormatFromPath('C:\\WSL\\ext4.vhdx')).toBe('Vhdx');
    expect(detectFormatFromPath('/vms/ubuntu.vmdk')).toBe('Vmdk');
    expect(detectFormatFromPath('/vms/windows.vdi')).toBe('Vdi');
    expect(detectFormatFromPath('archive.zip')).toBe('Unknown');
  });
});
