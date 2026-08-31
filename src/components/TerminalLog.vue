<template>
  <div v-if="logs.length > 0 || isCompacting" class="bg-gray-900 border border-gray-800 rounded-xl overflow-hidden shadow-2xl mb-8">
    <div class="bg-gray-950 px-4 py-3 border-b border-gray-800 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Terminal class="w-4 h-4 text-green-400" />
        <span class="text-xs font-mono font-bold text-gray-300">Live Execution Console</span>
      </div>
      <div v-if="isCompacting" class="flex items-center gap-2 text-xs font-mono text-sky-400">
        <Loader2 class="w-3.5 h-3.5 animate-spin" />
        <span class="animate-pulse">Compacting in progress...</span>
      </div>
    </div>

    <div
      ref="terminalContainer"
      class="p-4 font-mono text-xs max-h-60 overflow-y-auto bg-black/60 space-y-1.5 leading-relaxed"
    >
      <div v-if="logs.length === 0 && isCompacting" class="text-gray-500 italic">
        Initializing native command execution...
      </div>
      <div
        v-for="(log, idx) in logs"
        :key="idx"
        :class="log.is_error ? 'text-red-400 font-semibold' : 'text-gray-300'"
        class="flex items-start gap-2"
      >
        <span class="text-gray-600 select-none">&gt;</span>
        <span class="whitespace-pre-wrap break-all">{{ log.line }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { Terminal, Loader2 } from 'lucide-vue-next';
import type { CompactLogEvent } from '../types';

const props = defineProps<{
  logs: CompactLogEvent[];
  isCompacting: boolean;
}>();

const terminalContainer = ref<HTMLElement | null>(null);

watch(
  () => props.logs.length,
  async () => {
    await nextTick();
    if (terminalContainer.value) {
      terminalContainer.value.scrollTop = terminalContainer.value.scrollHeight;
    }
  }
);
</script>
