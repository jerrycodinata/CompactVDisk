<template>
  <div v-if="!isAdmin" class="bg-amber-900/40 border border-amber-600/50 rounded-lg p-4 text-amber-200 flex items-start gap-3 mb-6">
    <ShieldAlert class="w-6 h-6 text-amber-400 shrink-0 mt-0.5" />
    <div class="flex-1 text-sm">
      <div class="font-semibold text-amber-300 mb-1">Administrator Privileges Required</div>
      <p class="text-amber-200/90 leading-relaxed">
        Compacting WSL and VHDX disk images requires <code>diskpart</code> administrator rights. Please restart CompactVdisk as Administrator if compaction fails or encounters permission errors.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ShieldAlert } from 'lucide-vue-next';

defineProps<{
  isAdmin: boolean;
}>();
</script>
