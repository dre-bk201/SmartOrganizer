<template>
  <div class="dot" :class="[!$attrs.class]">
  </div>
</template>

<style scoped>
.dot {
  @apply h-[16px] w-[16px] rounded-full border-[3px];
}
</style>
