<template>
  <div class="bg-gray-800/80 border border-gray-700/60 rounded-xl p-5 shadow-lg mb-8">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-2.5">
        <HardDrive class="w-5 h-5 text-sky-400" />
        <h2 class="text-lg font-bold text-gray-100">Discovered Virtual Disks</h2>
        <span class="bg-sky-950 text-sky-400 border border-sky-800/50 text-xs px-2 py-0.5 rounded-full font-medium">
          {{ disks.length }} Detected
        </span>
      </div>
      <button
        @click="$emit('refresh')"
        :disabled="isCompacting"
        class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium bg-gray-700/60 hover:bg-gray-700 text-gray-200 rounded-lg transition disabled:opacity-50"
      >
        <RefreshCw class="w-3.5 h-3.5" />
        Refresh
      </button>
    </div>

    <div v-if="disks.length === 0" class="text-center py-8 text-gray-400 bg-gray-900/40 rounded-lg border border-dashed border-gray-700/50">
      <p class="text-sm">No WSL or Docker virtual disk files auto-detected on this system.</p>
      <p class="text-xs text-gray-500 mt-1">You can select custom .vhdx, .vmdk, or .vdi files below.</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div
        v-for="disk in disks"
        :key="disk.id as string"
        class="bg-gray-900/60 border border-gray-700/50 hover:border-gray-600 rounded-lg p-4 flex flex-col justify-between transition group"
      >
        <div>
          <div class="flex items-start justify-between gap-2 mb-2">
            <div>
              <span
                class="inline-block text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded mr-2"
                :class="disk.disk_type === 'Docker' ? 'bg-cyan-950 text-cyan-400 border border-cyan-800/60' : 'bg-purple-950 text-purple-400 border border-purple-800/60'"
              >
                {{ disk.disk_type }}
              </span>
              <span class="font-semibold text-gray-200 text-base">{{ disk.name }}</span>
            </div>
            <span class="text-xs font-mono font-medium px-2 py-0.5 rounded bg-gray-800 text-gray-300">
              {{ disk.format }}
            </span>
          </div>

          <p class="text-xs font-mono text-gray-400 truncate mb-3" :title="disk.path">
            {{ disk.path }}
          </p>
        </div>

        <div class="flex items-center justify-between pt-2 border-t border-gray-800">
          <div>
            <span class="text-xs text-gray-400 block">Current Size</span>
            <span class="text-sm font-bold text-sky-400 font-mono">{{ disk.size_formatted }}</span>
          </div>

          <div class="flex items-center gap-2">
            <button
              @click="$emit('show-details', disk)"
              class="flex items-center gap-1 px-2.5 py-1.5 text-xs font-medium bg-gray-800 hover:bg-gray-700 text-gray-300 hover:text-white border border-gray-700/60 rounded-md transition cursor-pointer"
            >
              <Info class="w-3.5 h-3.5 text-sky-400" />
              Details
            </button>
            <button
              @click="$emit('compact', disk)"
              :disabled="isCompacting"
              class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold bg-sky-600 hover:bg-sky-500 active:bg-sky-700 text-white rounded-md transition disabled:opacity-50 disabled:cursor-not-allowed shadow cursor-pointer"
            >
              <Zap class="w-3.5 h-3.5 fill-current" />
              Compact Disk
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { HardDrive, RefreshCw, Zap, Info } from 'lucide-vue-next';
import type { DiskInfo } from '../types';

defineProps<{
  disks: DiskInfo[];
  isCompacting: boolean;
}>();

defineEmits<{
  (e: 'refresh'): void;
  (e: 'compact', disk: DiskInfo): void;
  (e: 'show-details', disk: DiskInfo): void;
}>();
</script>
