<script lang="ts" setup>
import Icon from "./Icon.vue";

import { appWindow } from "@tauri-apps/api/window";
import { computed, inject, onMounted, ref } from "vue";
import { useStore } from "vuex";

const store = useStore();

const isDark = inject("isDark");

// Refs
const isMaximized = ref(false);

// Computed
const triggerClean = computed(() => store.getters.triggerClean);
const log = computed(() => store.getters["listeners/listener"].last);

// Functions

const close = () => appWindow.close();

const minimize = () => appWindow.minimize();

const maximize = () =>
  isMaximized.value ? appWindow.unmaximize() : appWindow.maximize();

const toggleTheme = () =>
  document.getElementById("app")?.classList.toggle("dark");

// Lifecycle Methods

onMounted(() => {
  appWindow.listen("tauri://resize", (q) => {
    appWindow.isMaximized().then((v) => (isMaximized.value = v));
  });
});
</script>

<template>
  <div
    data-tauri-drag-region
    class="h-8 bg-l_secondary dark:bg-d_secondary flex items-center pl-5 relative"
  >
    <span data-tauri-drag-region class="text-gray-800 dark:text-[#b6b6b6]">{{
      triggerClean ? log.path : ""
    }}</span>
    <Icon
      class="absolute right-36 hover:cursor-pointer"
      :fill="isDark ? '#b6b6b6' : 'black'"
      :key="(isDark as number)"
      @click="toggleTheme"
      name="settings"
      height="20"
      width="20"
    />

    <div class="flex h-full hover:cursor-pointer absolute right-0">
      <Icon
        @click="minimize"
        class="titlebar--icon right-36 px-3 center hover:bg-slate-500"
        :key="(isDark as number)"
        name="minimize"
        height="2"
        width="15"
        :fill="isDark ? '#b6b6b6' : 'black'"
      />

      <Icon
        @click="maximize"
        class="titlebar--icon px-3 center hover:bg-slate-500"
        :key="(isDark as number)"
        :name="isMaximized ? 'restore' : 'maximize'"
        height="15"
        width="15"
        :fill="isDark ? '#b6b6b6' : 'black'"
      />

      <Icon
        @click="close"
        class="titlebar--icon px-3 center hover:bg-red-600"
        :key="(isDark as number)"
        name="close"
        height="25"
        width="25"
        :fill="isDark ? '#b6b6b6' : 'black'"
      />
    </div>
  </div>
</template>
