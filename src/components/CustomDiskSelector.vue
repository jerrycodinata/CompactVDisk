<template>
  <div class="bg-gray-800/80 border border-gray-700/60 rounded-xl p-5 shadow-lg mb-8">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-2.5">
        <FolderPlus class="w-5 h-5 text-indigo-400" />
        <h2 class="text-lg font-bold text-gray-100">Custom Virtual Disk File Selector</h2>
      </div>
      <button
        @click="openNativePicker"
        :disabled="isCompacting"
        class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 text-white rounded-lg transition disabled:opacity-50"
      >
        <FileSearch class="w-3.5 h-3.5" />
        Browse Files
      </button>
    </div>

    <!-- Dropzone Area -->
    <div
      @dragover.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
      @drop.prevent="handleDrop"
      @click="openNativePicker"
      :class="[
        'border-2 border-dashed rounded-xl p-8 text-center cursor-pointer transition flex flex-col items-center justify-center gap-3',
        isDragging
          ? 'border-indigo-400 bg-indigo-950/30 text-indigo-300'
          : 'border-gray-700 hover:border-gray-500 bg-gray-900/30 text-gray-400 hover:text-gray-300'
      ]"
    >
      <UploadCloud :class="['w-10 h-10 transition', isDragging ? 'text-indigo-400 scale-110' : 'text-gray-500']" />
      <div>
        <p class="text-sm font-medium">
          Drag and drop disk image files here, or <span class="text-indigo-400 underline">browse</span>
        </p>
        <p class="text-xs text-gray-500 mt-1">
          Supported formats: <code class="text-gray-400">.vhdx</code>, <code class="text-gray-400">.vmdk</code>, <code class="text-gray-400">.vdi</code>
        </p>
      </div>
    </div>

    <!-- Custom Disks List -->
    <div v-if="customDisks.length > 0" class="mt-6 space-y-3">
      <h3 class="text-xs font-bold uppercase tracking-wider text-gray-400">Selected Disk Files</h3>
      <div
        v-for="disk in customDisks"
        :key="disk.id as string"
        class="bg-gray-900/60 border border-gray-700/50 rounded-lg p-3.5 flex items-center justify-between gap-4"
      >
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-xs font-mono font-bold uppercase px-2 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/60">
              {{ disk.format }}
            </span>
            <span class="font-medium text-gray-200 text-sm truncate">{{ disk.name }}</span>
            <span class="text-xs text-gray-400 font-mono font-semibold ml-auto shrink-0">
              {{ disk.size_formatted }}
            </span>
          </div>
          <p class="text-xs font-mono text-gray-500 truncate" :title="disk.path">{{ disk.path }}</p>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <button
            @click.stop="$emit('show-details', disk)"
            class="flex items-center gap-1 px-2.5 py-1.5 text-xs font-medium bg-gray-800 hover:bg-gray-700 text-gray-300 hover:text-white border border-gray-700/60 rounded-md transition cursor-pointer"
          >
            <Info class="w-3.5 h-3.5 text-sky-400" />
            Details
          </button>
          <button
            @click.stop="$emit('compact', disk)"
            :disabled="isCompacting"
            class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold bg-sky-600 hover:bg-sky-500 text-white rounded-md transition disabled:opacity-50 cursor-pointer"
          >
            <Zap class="w-3.5 h-3.5 fill-current" />
            Compact
          </button>
          <button
            @click.stop="$emit('remove', disk.path)"
            :disabled="isCompacting"
            class="p-1.5 text-gray-400 hover:text-red-400 hover:bg-red-950/30 rounded transition disabled:opacity-50 cursor-pointer"
            title="Remove from list"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { FolderPlus, FileSearch, UploadCloud, Zap, Trash2, Info } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-dialog';
import type { DiskInfo } from '../types';

defineProps<{
  customDisks: DiskInfo[];
  isCompacting: boolean;
}>();

const emit = defineEmits<{
  (e: 'add-path', path: string): void;
  (e: 'remove', path: string): void;
  (e: 'compact', disk: DiskInfo): void;
  (e: 'show-details', disk: DiskInfo): void;
}>();

const isDragging = ref(false);

async function openNativePicker() {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Virtual Disk Files (*.vhdx, *.vmdk, *.vdi)',
          extensions: ['vhdx', 'vmdk', 'vdi']
        }
      ]
    });

    if (selected && typeof selected === 'string') {
      emit('add-path', selected);
    }
  } catch (err) {
    console.error('File dialog error:', err);
  }
}

function handleDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    for (let i = 0; i < event.dataTransfer.files.length; i++) {
      const file = event.dataTransfer.files[i];
      // On Tauri desktop, file.path contains full native file path
      const nativePath = (file as any).path || file.name;
      emit('add-path', nativePath);
    }
  }
}
</script>
