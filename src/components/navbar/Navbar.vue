<script setup lang="ts">
import { computed, ref, watch } from "vue";

import NavItem from "./NavItem.vue";
import Logo from "../common/Logo.vue";
import SettingPanel from "../setting/SettingPanel.vue";

import { 
  SearchIcon, JournalIcon, PlusIcon, DashboardIcon, 
  StatisticsIcon, SettingIcon, LightIcon 
} from "@/icons";

import { inject } from "vue";

import type { TSettingStore } from "../../store";
import { ListenerModalKey, ListenerStoreKey } from "../../store/listener";
import { defaultListener } from "../../static";

const { settingStore } = defineProps<{ settingStore: TSettingStore }>();
const { listener } = inject(ListenerStoreKey)!;
const { open } = inject(ListenerModalKey)!;
const emits = defineEmits<{
  adjustWidth: [v: boolean];
}>();

let isPanelOpen = ref(false);
let isPinned = ref(settingStore.pin);

const pin = computed(() => settingStore.pin);
const isLight = computed(() => !settingStore.isDark);
let search = computed({
  get() {
    return listener.search;
  },
  set(v) {
    listener.setSearch(v);
  },
});

watch(isPinned, (p) => {
  emits("adjustWidth", p);
});

watch(pin, (p) => {
  if (isPanelOpen.value && pin.value) {
  } else isPinned.value = p;
});

function handleFocus(e: FocusEvent) {
  if (e.type == "focusin") isPinned.value = false;
  else if (e.type == "focusout") isPinned.value = pin.value;
}

async function openAddListenerModal() {
  let l = defaultListener();
  await listener.addListener(l);
  open(l.id);
}

function toggleSettingsPanel() {
  isPanelOpen.value = !isPanelOpen.value;
  if (isPanelOpen.value) isPinned.value = false;
  else isPinned.value = pin.value;
}
</script>

<template>
  <nav data-tauri-drag-region class="h-full flex flex-col items-center relative adaptive--dark">
    <SettingPanel v-if="isPanelOpen" :close="toggleSettingsPanel" />

    <section class="px-4 flex-shrink-0">
      <Logo :isPinned="isPinned" />

      <div class="searchbox" :class="[isPinned ? 'searchbox--pinned' : 'searchbox--unpinned']">
        <SearchIcon class="pointer-events-none" />
        <input class="adaptive--darker" @focusin="handleFocus" @focusout="handleFocus" v-model="search" type="text"
          :placeholder="!isPinned ? 'Search' : ''" />
      </div>
    </section>

    <section class="w-full flex-col flex gap-4 px-4 flex-shrink-0">
      <NavItem :isPinned="isPinned" to="/" name="Dashboard">
        <template #icon="{ isActive }">
          <DashboardIcon class="icon" :class="[isActive && 'text-blue-500']" />
        </template>

        <span>Dashboard</span>
      </NavItem>

      <NavItem :isPinned="isPinned" to="/journal" name="Journal">
        <template #icon="{ isActive }">
          <JournalIcon class="icon" :class="[isActive && 'text-blue-500']" />
        </template>

        <span>Journal</span>
      </NavItem>

      <NavItem :isPinned="isPinned" to="/statistics" name="Statistics">
        <template #icon="{ isActive }">
          <StatisticsIcon class="icon" :class="[isActive && 'text-blue-500']" />
        </template>

        <span>Statistics</span>
      </NavItem>
    </section>

    <div class="bg-gray-400 my-8 w-full h-[1px] flex-shrink-0" />

    <section class="relative flex-grow">
      <div class="text-gray-700 text-xl dark:text-gray-200 mt-3 items-start flex gap-10 flex-col">
        <!--Add New Organizer-->
        <span class="flex items-center hover:cursor-pointer gap-3" @click="openAddListenerModal">
          <span class="animate-bounce hover:animate-none duration-1000 rounded-lg outline outline-2 dark:outline-gray-200">
            <PlusIcon class="text-2xl" />
          </span>
          <span v-if="!isPinned" class="text-base"> Add New Organizer </span>
        </span>
        <!--Add New Organizer-->

        <!-- Settings -->
        <span @click="toggleSettingsPanel" class="flex gap-3 items-center">
          <SettingIcon class="text-2xl" />
          <span v-show="!isPinned" class="text-base">Settings</span>
        </span>
        <!-- Settings -->
      </div>

      <!-- THEMESWITCH -->
      <div @click="settingStore.toggleTheme" class="absolute bottom-5 hover:cursor-pointer flex items-center gap-3">
        <span class="dark:bg-darker bg-lighter p-[7px] rounded-lg outline outline-2 outline-gray-500">
          <LightIcon class="text-blue-500" />
        </span>

        <span v-show="!isPinned" class="text-md font-bold dark:text-gray-400 text-gray-800">
          <span :class="[isLight && 'text-blue-500']"> LIGHT </span>
          <span class="text-xl"> / </span>
          <span :class="[!isLight && 'text-blue-500']"> DARK </span>
        </span>
      </div>
      <!-- THEMESWITCH -->
    </section>
  </nav>
</template>

<style scoped>
.icon {
  @apply text-xl box-content;
}

.searchbox {
  @apply relative mb-5 flex items-center h-11 w-full;
}

.searchbox input {
  @apply text-gray-200 indent-9 border-gray-500 focus:border-blue-500 outline-none border-[1px] rounded-md;
}

.searchbox svg {
  @apply text-blue-500 absolute;
}

.searchbox--unpinned svg {
  @apply left-3;
}

.searchbox--unpinned input {
  @apply w-full h-full;
}

.searchbox--pinned {
  @apply justify-center px-1;
}

.searchbox--pinned input {
  @apply w-full h-full;
}
</style>
