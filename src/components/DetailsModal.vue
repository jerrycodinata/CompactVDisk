<template>
  <div
    v-if="disk"
    class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4 animate-fade-in"
    @click.self="$emit('close')"
  >
    <div class="bg-gray-800 border border-gray-700 rounded-2xl max-w-xl w-full p-6 shadow-2xl relative overflow-hidden">
      <!-- Header -->
      <div class="flex items-start justify-between border-b border-gray-700/60 pb-4 mb-5">
        <div class="flex items-center gap-3">
          <div class="p-2.5 bg-sky-950/80 border border-sky-800/60 rounded-xl text-sky-400">
            <Info class="w-6 h-6" />
          </div>
          <div>
            <h2 class="text-lg font-bold text-gray-100 flex items-center gap-2">
              {{ disk.name }}
            </h2>
            <p class="text-xs text-gray-400">Virtual Disk Details</p>
          </div>
        </div>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-gray-200 p-1 rounded-lg hover:bg-gray-700/50 transition cursor-pointer"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content -->
      <div class="space-y-4">
        <!-- Grid Specs -->
        <div class="grid grid-cols-2 gap-3">
          <div class="bg-gray-900/60 border border-gray-700/50 rounded-xl p-3">
            <span class="text-xs text-gray-400 block mb-1">Disk Format</span>
            <span class="text-sm font-semibold font-mono text-sky-400">{{ disk.format }}</span>
          </div>

          <div class="bg-gray-900/60 border border-gray-700/50 rounded-xl p-3">
            <span class="text-xs text-gray-400 block mb-1">Disk Type</span>
            <span class="text-sm font-semibold text-purple-400">{{ disk.disk_type }}</span>
          </div>

          <div class="bg-gray-900/60 border border-gray-700/50 rounded-xl p-3">
            <span class="text-xs text-gray-400 block mb-1">Current Size</span>
            <span class="text-sm font-semibold font-mono text-emerald-400">{{ disk.size_formatted }}</span>
            <span class="text-[10px] text-gray-500 block font-mono">({{ disk.size_bytes.toLocaleString() }} bytes)</span>
          </div>

          <div class="bg-gray-900/60 border border-gray-700/50 rounded-xl p-3">
            <span class="text-xs text-gray-400 block mb-1">Status / State</span>
            <span class="text-sm font-semibold text-amber-300">{{ disk.status || 'Ready' }}</span>
          </div>
        </div>

        <!-- File Path with Copy Button -->
        <div class="bg-gray-900/60 border border-gray-700/50 rounded-xl p-3.5">
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-xs text-gray-400 font-medium">Full File Path</span>
            <button
              @click="copyPath"
              class="text-xs text-sky-400 hover:text-sky-300 flex items-center gap-1 bg-sky-950/60 border border-sky-800/50 px-2 py-0.5 rounded transition cursor-pointer"
            >
              <Copy v-if="!copied" class="w-3 h-3" />
              <Check v-else class="w-3 h-3 text-emerald-400" />
              {{ copied ? 'Copied!' : 'Copy Path' }}
            </button>
          </div>
          <p class="text-xs font-mono text-gray-300 break-all bg-gray-950 p-2 rounded border border-gray-800">
            {{ disk.path }}
          </p>
        </div>

        <!-- Compaction Technical Info -->
        <div class="bg-gray-900/40 border border-gray-800 rounded-xl p-3.5 text-xs text-gray-400 space-y-1">
          <div class="font-semibold text-gray-300 mb-1 flex items-center gap-1.5">
            <Wrench class="w-3.5 h-3.5 text-sky-400" /> Compaction Method
          </div>
          <p v-if="disk.format === 'Vhdx'">
            Compacted using Windows <code class="text-sky-300 font-mono">diskpart</code> utility with <code class="text-sky-300 font-mono">compact vdisk</code> command. Requires administrator privileges.
          </p>
          <p v-else-if="disk.format === 'Vdi'">
            Compacted using Oracle VirtualBox <code class="text-sky-300 font-mono">VBoxManage modifymedium disk --compact</code> command.
          </p>
          <p v-else-if="disk.format === 'Vmdk'">
            Compacted using VMware <code class="text-sky-300 font-mono">vmware-vdiskmanager -k</code> or <code class="text-sky-300 font-mono">qemu-img convert</code>.
          </p>
          <p v-else>
            Custom or unknown format. Supported formats for storage compaction are VHDX, VMDK, and VDI.
          </p>
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="mt-6 flex justify-end gap-3 pt-4 border-t border-gray-700/60">
        <button
          @click="$emit('close')"
          class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-gray-200 text-xs font-semibold rounded-lg transition cursor-pointer"
        >
          Close
        </button>
        <button
          @click="$emit('compact', disk); $emit('close')"
          class="px-4 py-2 bg-sky-600 hover:bg-sky-500 text-white text-xs font-semibold rounded-lg transition flex items-center gap-1.5 shadow cursor-pointer"
        >
          <Zap class="w-3.5 h-3.5 fill-current" />
          Compact This Disk
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Info, X, Copy, Check, Wrench, Zap } from 'lucide-vue-next';
import type { DiskInfo } from '../types';

const props = defineProps<{
  disk: DiskInfo | null;
}>();

defineEmits<{
  (e: 'close'): void;
  (e: 'compact', disk: DiskInfo): void;
}>();

const copied = ref(false);

async function copyPath() {
  if (!props.disk) return;
  try {
    await navigator.clipboard.writeText(props.disk.path);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (e) {
    console.error('Failed to copy path:', e);
  }
}
</script>
