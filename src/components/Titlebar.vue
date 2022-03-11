<script lang="ts" setup>
import Icon from "./Icon.vue";

import { computed, ComputedRef, inject, ref, toRefs } from "vue";
import { useStore } from "vuex";
import { Log } from "../store/modules/listener";
import { useDimensions } from "../utils";

const store = useStore();
const dimensions = useDimensions();

const isDark = inject("isDark");
const log: ComputedRef<Log> | undefined = inject("logDetail");

// Refs
const { isMaximized, appWindow } = toRefs(dimensions);

// Computed
const isCleaning = computed(() => store.getters.triggerClean);
const osType: ComputedRef<"win32" | "macos"> = computed(
  () => store.getters["settings/titlebar"]
);
// Functions
const close = () => appWindow.value.close();

const minimize = () => appWindow.value.minimize();

const toggleMaximize = () => appWindow.value.toggleMaximize();

const toggleTheme = () =>
  document.getElementById("app")?.classList.toggle("dark");
</script>

<template>
  <div
    v-if="osType == 'win32'"
    data-tauri-drag-region
    class="min-h-[32px] max-h-[32px] bg-l_secondary dark:bg-d_secondary flex items-center pl-5 relative"
  >
    <span data-tauri-drag-region class="text-gray-800 dark:text-[#b6b6b6]">
      {{ isCleaning ? "Path: " + log?.path : "" }}
    </span>
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
        @click="toggleMaximize"
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
  <div
    v-else-if="osType == 'macos'"
    data-tauri-drag-region
    class="min-h-[32px] max-h-[32px] bg-l_secondary dark:bg-d_secondary flex items-center pl-5"
  >
    <div class="flex items-center justify-around absolute left-1 w-[80.4px]">
      <div
        @click="close"
        class="hover:cursor-pointer bg-[#ff5259] w-[13.8px] h-[13.8px] rounded-full"
      />
      <div
        @click="minimize"
        class="hover:cursor-pointer bg-[#ffbe2d] w-[13.8px] h-[13.8px] rounded-full"
      />
      <div
        @click="toggleMaximize"
        class="hover:cursor-pointer bg-[#27ca40] w-[13.8px] h-[13.8px] rounded-full"
      />
    </div>

    <Icon
      class="absolute left-28 hover:cursor-pointer"
      :fill="isDark ? '#b6b6b6' : 'black'"
      :key="(isDark as number)"
      @click="toggleTheme"
      name="settings"
      height="20"
      width="20"
    />

    <span
      data-tauri-drag-region
      class="text-gray-800 absolute left-36 dark:text-[#b6b6b6]"
    >
      {{ isCleaning ? "Path: " + log?.path : "" }}
    </span>
  </div>
</template>
