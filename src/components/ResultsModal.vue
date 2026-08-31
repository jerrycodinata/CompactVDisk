<template>
  <div
    v-if="isOpen && result"
    class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div class="bg-gray-800 border border-gray-700 rounded-2xl max-w-md w-full p-6 shadow-2xl relative animate-in fade-in zoom-in duration-150">
      <button
        @click="$emit('close')"
        class="absolute top-4 right-4 text-gray-400 hover:text-white p-1 rounded-lg transition"
      >
        <X class="w-5 h-5" />
      </button>

      <div class="flex items-center gap-3 mb-4">
        <div class="p-3 bg-emerald-950/80 border border-emerald-800/60 rounded-xl text-emerald-400">
          <Sparkles class="w-6 h-6" />
        </div>
        <div>
          <h3 class="text-lg font-bold text-gray-100">Compaction Complete</h3>
          <p class="text-xs text-gray-400">Virtual disk storage reclaimed successfully</p>
        </div>
      </div>

      <div class="space-y-3 my-6">
        <div class="bg-gray-900/80 border border-gray-800 rounded-xl p-4 flex items-center justify-between">
          <span class="text-xs text-gray-400 font-medium">Initial Disk Size</span>
          <span class="text-sm font-mono font-semibold text-gray-300">{{ result.initial_size_formatted }}</span>
        </div>

        <div class="bg-gray-900/80 border border-gray-800 rounded-xl p-4 flex items-center justify-between">
          <span class="text-xs text-gray-400 font-medium">New Disk Size</span>
          <span class="text-sm font-mono font-semibold text-sky-400">{{ result.new_size_formatted }}</span>
        </div>

        <div class="bg-emerald-950/40 border border-emerald-800/60 rounded-xl p-4 flex items-center justify-between">
          <span class="text-xs text-emerald-300 font-bold uppercase tracking-wider">Reclaimed Space</span>
          <span class="text-base font-mono font-extrabold text-emerald-400">Saved {{ result.reclaimed_formatted }}</span>
        </div>

        <div class="text-right text-xs text-gray-500 font-mono">
          Elapsed Time: {{ result.elapsed_seconds.toFixed(2) }} seconds
        </div>
      </div>

      <button
        @click="$emit('close')"
        class="w-full py-2.5 px-4 bg-sky-600 hover:bg-sky-500 text-white text-sm font-bold rounded-xl transition shadow-lg"
      >
        Done
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { X, Sparkles } from 'lucide-vue-next';
import type { CompactionResult } from '../types';

defineProps<{
  isOpen: boolean;
  result: CompactionResult | null;
}>();

defineEmits<{
  (e: 'close'): void;
}>();
</script>
