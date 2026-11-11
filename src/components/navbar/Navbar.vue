<script setup lang="ts">
import { computed, inject, ref, watch } from 'vue';
import NavItem from './NavItem.vue';
import SettingPanel from '../setting/SettingPanel.vue';
import { SearchIcon, JournalIcon, PlusIcon, DashboardIcon, StatisticsIcon, SettingIcon, LightIcon } from '@/icons';
import type { TSettingStore } from '../../store';
import { ListenerModalKey, ListenerStoreKey } from '../../store/listener';
import { defaultListener } from '../../static';
const { settingStore } = defineProps<{ settingStore: TSettingStore }>();
const { listener } = inject(ListenerStoreKey)!;
const { open } = inject(ListenerModalKey)!;
const emits = defineEmits<{ adjustWidth: [v: boolean] }>();
const isPanelOpen = ref(false);
const isPinned = computed(() => settingStore.pin && !isPanelOpen.value);
watch(isPinned, value => emits('adjustWidth', value), { immediate: true });
const search = computed({ get: () => listener.search, set: value => listener.setSearch(value) });
const activeCount = computed(() => listener.listeners.filter(item => item.enabled).length);
async function addListener() { const item = defaultListener(); await listener.addListener(item); open(item.id); }
</script>
<template>
  <nav class="flow-sidebar" :class="{ collapsed: isPinned }" aria-label="Main navigation">
    <SettingPanel v-if="isPanelOpen" :close="() => isPanelOpen = false" />
    <div class="brand" data-tauri-drag-region><span class="brand-mark" aria-hidden="true"></span><span v-if="!isPinned">SmartOrganizer</span></div>
    <label class="sidebar-search"><SearchIcon aria-hidden="true"/><input v-model="search" aria-label="Search listeners" :placeholder="isPinned ? '' : 'Find a listener…'" /></label>
    <div class="navlinks">
      <NavItem :is-pinned="isPinned" to="/" name="Dashboard"><template #icon><DashboardIcon /></template></NavItem>
      <NavItem :is-pinned="isPinned" to="/journal" name="Journal"><template #icon><JournalIcon /></template></NavItem>
      <NavItem :is-pinned="isPinned" to="/statistics" name="Statistics"><template #icon><StatisticsIcon /></template></NavItem>
    </div>
    <button class="new-listener" :aria-label="isPinned ? 'New listener' : undefined" @click="addListener"><PlusIcon aria-hidden="true"/><span v-if="!isPinned">New listener</span></button>
    <div class="sidebar-bottom">
      <button class="sidebar-action" :aria-expanded="isPanelOpen" aria-label="Settings" @click="isPanelOpen = !isPanelOpen"><SettingIcon aria-hidden="true"/><span v-if="!isPinned">Settings</span></button>
      <button class="sidebar-action" :aria-label="settingStore.isDark ? 'Switch to light theme' : 'Switch to dark theme'" @click="settingStore.toggleTheme"><LightIcon aria-hidden="true"/><span v-if="!isPinned">{{ settingStore.isDark ? 'Light appearance' : 'Dark appearance' }}</span></button>
      <p v-if="!isPinned" class="watching"><span class="status-dot"></span>{{ activeCount }} {{ activeCount === 1 ? 'listener' : 'listeners' }} enabled</p>
    </div>
  </nav>
</template>
<style scoped>
.flow-sidebar{height:100%;display:flex;flex-direction:column;position:relative;padding:20px 10px 16px 6px;background:var(--flow-surface,#f5edef);color:var(--flow-text,#422b32);border-right:1px solid var(--flow-border,#e8d9dd);gap:20px}.brand{display:flex;align-items:center;gap:8px;font-size:13px;font-weight:650;white-space:nowrap;padding:0 2px}.brand-mark{width:24px;height:24px;border:6px solid var(--flow-accent,#a32949);border-radius:6px;transform:rotate(-8deg);flex-shrink:0}.sidebar-search{display:flex;align-items:center;gap:8px;padding:9px 10px;border:1px solid var(--flow-border,#e8d9dd);background:var(--flow-bg,#fcf9fa);border-radius:6px;color:var(--flow-muted,#80616b)}.sidebar-search svg{width:16px;flex-shrink:0}.sidebar-search input{width:100%;min-width:0;background:transparent;outline:0;font-size:13px;color:var(--flow-text,#422b32)}.sidebar-search:focus-within{outline:2px solid var(--flow-accent,#a32949);outline-offset:2px}.navlinks{display:flex;flex-direction:column;gap:5px}.new-listener,.sidebar-action{display:flex;align-items:center;gap:8px;font-size:13px;padding:10px 8px;border-radius:6px;text-align:left}.new-listener{background:var(--flow-accent,#a32949);color:var(--flow-accent-ink,#fff);justify-content:center}.new-listener svg,.sidebar-action svg{width:17px;height:17px;flex-shrink:0}.sidebar-bottom{margin-top:auto;display:flex;flex-direction:column;gap:5px}.sidebar-action{color:var(--flow-muted,#80616b)}.sidebar-action:hover{background:var(--flow-bg,#fcf9fa);color:var(--flow-text,#422b32)}.watching{font-size:12px;color:var(--flow-muted,#80616b);margin-top:22px;padding:0 8px}.status-dot{display:inline-block;width:6px;height:6px;border-radius:50%;background:var(--flow-accent,#a32949);margin-right:7px}.collapsed{padding:20px 6px 16px}.collapsed .brand{justify-content:center}.collapsed .sidebar-search{padding:9px 6px;gap:0}.collapsed .sidebar-search input{width:0}.collapsed .sidebar-search:focus-within input{width:20px}.collapsed .sidebar-search:focus-within svg{display:none}.collapsed .sidebar-action{justify-content:center}.collapsed .new-listener{padding:10px}
</style>
