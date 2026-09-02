<template>
  <div v-if="!isAdmin" class="bg-amber-900/40 border border-amber-600/50 rounded-lg p-4 text-amber-200 flex items-center justify-between gap-3 mb-6">
    <div class="flex items-start gap-3">
      <ShieldAlert class="w-6 h-6 text-amber-400 shrink-0 mt-0.5" />
      <div class="flex-1 text-sm">
        <div class="font-semibold text-amber-300 mb-1">Administrator Privileges Required</div>
        <p class="text-amber-200/90 leading-relaxed">
          Compacting WSL and VHDX virtual disks requires administrator rights. Please restart CompactVdisk as Administrator.
        </p>
      </div>
    </div>
    <button
      @click="$emit('relaunch')"
      class="shrink-0 px-3.5 py-2 bg-amber-600 hover:bg-amber-500 active:bg-amber-700 text-white font-medium text-xs rounded-md shadow transition flex items-center gap-1.5 cursor-pointer"
    >
      <ShieldCheck class="w-4 h-4" />
      Restart as Admin
    </button>
  </div>
</template>

<script setup lang="ts">
import { ShieldAlert, ShieldCheck } from 'lucide-vue-next';

defineProps<{
  isAdmin: boolean;
}>();

defineEmits<{
  (e: 'relaunch'): void;
}>();
</script>
