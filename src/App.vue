<template>
  <div class="min-h-screen bg-gray-900 text-gray-100 flex flex-col font-sans selection:bg-sky-500 selection:text-white">
    <!-- Top Header -->
    <header class="border-b border-gray-800 bg-gray-950/80 backdrop-blur sticky top-0 z-40 px-6 py-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-sky-600 rounded-lg text-white shadow-md">
          <Disc3 class="w-6 h-6 animate-spin-slow" />
        </div>
        <div>
          <h1 class="text-xl font-bold tracking-tight text-white flex items-center gap-2">
            CompactVdisk
            <span class="text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded bg-sky-950 text-sky-400 border border-sky-800/60">
              v2.0
            </span>
          </h1>
          <p class="text-xs text-gray-400">Virtual disk storage reclamation utility</p>
        </div>
      </div>

      <!-- Quick Tool Status Indicators -->
      <div v-if="toolAvailability" class="hidden md:flex items-center gap-2 text-xs font-mono">
        <span class="text-gray-500">System Tools:</span>
        <span
          :class="toolAvailability.diskpart ? 'bg-green-950 text-green-400 border-green-800/60' : 'bg-gray-800 text-gray-500 border-gray-700'"
          class="px-2 py-0.5 rounded border"
        >
          diskpart
        </span>
        <span
          :class="toolAvailability.qemu_img ? 'bg-green-950 text-green-400 border-green-800/60' : 'bg-gray-800 text-gray-500 border-gray-700'"
          class="px-2 py-0.5 rounded border"
        >
          qemu-img
        </span>
        <span
          :class="toolAvailability.vboxmanage ? 'bg-green-950 text-green-400 border-green-800/60' : 'bg-gray-800 text-gray-500 border-gray-700'"
          class="px-2 py-0.5 rounded border"
        >
          vboxmanage
        </span>
      </div>
    </header>

    <!-- Main Content Area -->
    <main class="flex-1 max-w-5xl w-full mx-auto p-6">
      <AdminBanner :is-admin="isAdmin" @relaunch="relaunchAsAdmin" />

      <div v-if="errorMsg" class="bg-red-950/60 border border-red-800/80 rounded-xl p-4 text-red-200 text-sm mb-6 flex items-start justify-between gap-3">
        <div class="flex items-center gap-2">
          <AlertTriangle class="w-5 h-5 text-red-400 shrink-0" />
          <span>{{ errorMsg }}</span>
        </div>
        <button @click="errorMsg = null" class="text-xs text-red-400 underline">Dismiss</button>
      </div>

      <AutoDiscoveryDashboard
        :disks="discoveredDisks"
        :is-compacting="isCompacting"
        @refresh="fetchDisks"
        @compact="startCompaction"
        @show-details="selectedDiskForDetails = $event"
      />

      <CustomDiskSelector
        :custom-disks="customDisks"
        :is-compacting="isCompacting"
        @add-path="handleAddPath"
        @remove="removeCustomDisk"
        @compact="startCompaction"
        @show-details="selectedDiskForDetails = $event"
      />

      <TerminalLog
        :logs="logs"
        :is-compacting="isCompacting"
      />
    </main>

    <!-- Results Modal -->
    <ResultsModal
      :is-open="isResultModalOpen"
      :result="lastResult"
      @close="closeResultModal"
    />

    <!-- Disk Details Modal -->
    <DetailsModal
      :disk="selectedDiskForDetails"
      @close="selectedDiskForDetails = null"
      @compact="startCompaction"
    />

    <footer class="border-t border-gray-800 py-4 px-6 text-center text-xs text-gray-500">
      CompactVdisk &copy; {{ new Date().getFullYear() }} &bull; Tauri v2 + Vue 3 + Tailwind CSS
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Disc3, AlertTriangle } from 'lucide-vue-next';
import { useDiskStore } from './stores/diskStore';
import type { DiskInfo } from './types';

import AdminBanner from './components/AdminBanner.vue';
import AutoDiscoveryDashboard from './components/AutoDiscoveryDashboard.vue';
import CustomDiskSelector from './components/CustomDiskSelector.vue';
import TerminalLog from './components/TerminalLog.vue';
import ResultsModal from './components/ResultsModal.vue';
import DetailsModal from './components/DetailsModal.vue';

const selectedDiskForDetails = ref<DiskInfo | null>(null);

const {
  discoveredDisks,
  customDisks,
  isAdmin,
  toolAvailability,
  isCompacting,
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
} = useDiskStore();

async function handleAddPath(path: string) {
  try {
    await inspectAndAddCustomDisk(path);
  } catch (e) {
    console.error('Failed to add custom disk path:', e);
  }
}

onMounted(async () => {
  await checkAdminStatus();
  await checkTools();
  await fetchDisks();
});
</script>
