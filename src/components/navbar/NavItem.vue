<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
const route = useRoute();
const props = defineProps<{ to: string; name: string; isPinned: boolean }>();
const isActive = computed(() => route.name === props.name);
</script>
<template>
  <router-link :to="to" class="navitem" :class="{ active: isActive, collapsed: isPinned }" :aria-label="isPinned ? name : undefined" :title="isPinned ? name : undefined" :aria-current="isActive ? 'page' : undefined">
    <span class="nav-icon"><slot name="icon" :is-active="isActive" /></span><span v-if="!isPinned"><slot>{{ name }}</slot></span>
  </router-link>
</template>
<style scoped>
.navitem{display:flex;align-items:center;gap:8px;padding:11px 8px;border-radius:6px;font-size:13px;color:var(--flow-muted,#80616b);text-decoration:none}.navitem:hover{background:var(--flow-bg,#fcf9fa)}.navitem.active{background:var(--flow-bg,#fcf9fa);color:var(--flow-accent,#a32949);font-weight:650}.nav-icon{display:flex;width:18px;height:18px;flex-shrink:0}.nav-icon :deep(svg){width:18px;height:18px}.collapsed{justify-content:center;padding:11px 8px}
</style>
