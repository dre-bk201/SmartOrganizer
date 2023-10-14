<script setup lang="ts">
//@ts-nocheck
import { TSettingStore } from "../../store";
import { computed } from "vue";

import CloseIcon from "~icons/close";
import MinimizeIcon from "~icons/minimize";
import MaximizeIcon from "~icons/maximize";

import {
  WindowTitlebarProps,
  WindowTitlebar,
  WindowControls,
} from "./tauri-controls";

const { settingStore } = defineProps<{ settingStore: TSettingStore }>();

const style = computed(() => settingStore.titlebarStyle);
</script>

<template>
  <div class="py-[3px] adaptive--dark">
    <WindowTitlebar :key="style" :window-controls-props="{
      justify: true,
      platform: style,
      className: 'h-7 min-h-7',
    }">
    </WindowTitlebar>
  </div>
</template>
