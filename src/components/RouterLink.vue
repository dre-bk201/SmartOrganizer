<script lang="ts" setup>
import { useRoute } from "vue-router";

const route = useRoute();

defineProps<{ to: string }>();
</script>

<template>
  <router-link :to="to">
    <div
      :class="`w-3 h-3 rounded-full ${
        route.path == to ? 'bg-[#FF3B76]' : 'bg-[#909090]'
      }`"
    />
  </router-link>
</template>
