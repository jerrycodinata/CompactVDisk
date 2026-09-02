import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { DiskInfo, ToolAvailability, CompactionResult, CompactLogEvent } from '../types';

export function useDiskStore() {
  const discoveredDisks = ref<DiskInfo[]>([]);
  const customDisks = ref<DiskInfo[]>([]);
  const isAdmin = ref<boolean>(true);
  const toolAvailability = ref<ToolAvailability | null>(null);

  const isCompacting = ref<boolean>(false);
  const currentCompactingDiskId = ref<string | null>(null);
  const logs = ref<CompactLogEvent[]>([]);
  const lastResult = ref<CompactionResult | null>(null);
  const isResultModalOpen = ref<boolean>(false);
  const errorMsg = ref<string | null>(null);

  let unlistenLog: UnlistenFn | null = null;

  async function checkAdminStatus() {
    try {
      isAdmin.value = await invoke<boolean>('check_admin');
    } catch {
      isAdmin.value = false;
    }
  }

  async function relaunchAsAdmin() {
    try {
      await invoke<boolean>('relaunch_as_admin');
    } catch (e) {
      console.error('Failed to relaunch as admin:', e);
    }
  }

  async function checkTools() {
    try {
      toolAvailability.value = await invoke<ToolAvailability>('check_tools');
    } catch {
      toolAvailability.value = null;
    }
  }

  async function fetchDisks() {
    try {
      discoveredDisks.value = await invoke<DiskInfo[]>('discover_disks');
    } catch (e) {
      console.error('Failed to discover disks:', e);
    }
  }

  async function inspectAndAddCustomDisk(path: string) {
    try {
      const disk = await invoke<DiskInfo>('inspect_disk', { path });
      if (!customDisks.value.some(d => d.path === disk.path)) {
        customDisks.value.push(disk);
      }
      return disk;
    } catch (e: any) {
      errorMsg.value = String(e);
      throw e;
    }
  }

  function removeCustomDisk(path: string) {
    customDisks.value = customDisks.value.filter(d => d.path !== path);
  }

  async function startCompaction(disk: DiskInfo) {
    if (isCompacting.value) return;

    isCompacting.value = true;
    currentCompactingDiskId.value = disk.id as string;
    logs.value = [];
    lastResult.value = null;
    errorMsg.value = null;

    try {
      unlistenLog = await listen<CompactLogEvent>('compact-log', (event) => {
        logs.value.push(event.payload);
      });

      const res = await invoke<CompactionResult>('compact_disk', {
        diskId: disk.id,
        path: disk.path,
      });

      lastResult.value = res;
      isResultModalOpen.value = true;

      await fetchDisks();
      for (let i = 0; i < customDisks.value.length; i++) {
        if (customDisks.value[i].path === disk.path) {
          try {
            const updated = await invoke<DiskInfo>('inspect_disk', { path: disk.path });
            customDisks.value[i] = updated;
          } catch (_) {}
        }
      }
    } catch (err: any) {
      errorMsg.value = String(err);
    } finally {
      isCompacting.value = false;
      currentCompactingDiskId.value = null;
      if (unlistenLog) {
        unlistenLog();
        unlistenLog = null;
      }
    }
  }

  function closeResultModal() {
    isResultModalOpen.value = false;
  }

  return {
    discoveredDisks,
    customDisks,
    isAdmin,
    toolAvailability,
    isCompacting,
    currentCompactingDiskId,
    logs,
    lastResult,
    isResultModalOpen,
    errorMsg,
    checkAdminStatus,
    relaunchAsAdmin,
    checkTools,
    fetchDisks,
    inspectAndAddCustomDisk,
    removeCustomDisk,
    startCompaction,
    closeResultModal,
  };
}
