<script setup lang="ts">
import { computed, ref } from 'vue';
import type { TSettingStore } from '../../store';
const { settingStore } = defineProps<{ settingStore: TSettingStore }>();
const pin = ref(settingStore.pin);
const radius = computed(() => settingStore.rounded);
</script>
<template>
  <div class="flow-app acrylic-shell">
    <slot name="titlebar" />
    <div class="flow-body">
      <slot name="listenerModal" :radius="radius" />
      <aside class="flow-navigation" :class="{ collapsed: pin }"><slot name="navbar" :adjust-width="(value: boolean) => pin = value" /></aside>
      <section class="flow-content"><slot /></section>
    </div>
    <slot name="modal" />
  </div>
</template>
<style scoped>
.flow-app{height:100vh;display:flex;flex-direction:column;overflow:hidden;background:var(--flow-bg,#fcf9fa);color:var(--flow-text,#422b32)}.flow-body{display:flex;flex:1;min-height:0;position:relative}.flow-navigation{width:176px;flex-shrink:0}.flow-navigation.collapsed{width:56px}.flow-content{display:flex;flex-direction:column;flex:1;min-width:0;min-height:0;overflow:hidden;background:var(--flow-bg,#fcf9fa)}@media(max-width:760px){.flow-navigation{width:160px}}@media(prefers-reduced-motion:no-preference){.flow-navigation{transition:width 180ms ease-out}}
</style>
