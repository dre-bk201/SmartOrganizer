<script setup lang="ts">

defineProps<{ state: boolean; first: string; second: string }>()
</script>


<template>
  <div class="w-full gap-4 transition-all flex items-center justify-center">
    <div class="dark:bg-darker bg-lighter shadow shadow-dark p-1 relative items-center h-[45px] rounded-md w-[80px]">
      <slot></slot>
    </div>

    <span class="text-sm font-bold dark:text-gray-400 text-gray-800">
      <span :class="[state && 'text-blue-500']"> {{ first }} </span>
      <span class="text-xl"> / </span>
      <span :class="[!state && 'text-blue-500']"> {{ second }} </span>
    </span>
  </div>
</template>
