<script lang="ts" setup>
import Icon from "./Icon.vue";

import { appWindow } from "@tauri-apps/api/window";
import { inject, onMounted, ref } from "vue";

const isDark = inject("isDark");

const isMaximized = ref(false);

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
    class="h-8 bg-l_secondary dark:bg-d_secondary flex items-center justify-end relative"
  >
    <Icon
      class="absolute right-36 hover:cursor-pointer"
      :fill="isDark ? '#b6b6b6' : 'black'"
      :key="(isDark as number)"
      @click="toggleTheme"
      name="settings"
      height="20"
      width="20"
    />

    <div class="flex h-full hover:cursor-pointer">
      <Icon
        @click="minimize"
        class="titlebar--icon px-3 center hover:bg-slate-500"
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
