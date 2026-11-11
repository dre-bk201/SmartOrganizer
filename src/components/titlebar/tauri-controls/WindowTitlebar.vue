<script setup lang="ts">
import type { OsType } from "@tauri-apps/plugin-os";
import { twMerge } from "tailwind-merge"
import { onMounted, ref, computed } from "vue"
import type { WindowControlsProps, WindowTitlebarProps } from "./types"
import { getOsType } from "./utils/os"
import WindowControls from "./WindowControls.vue"

const props = withDefaults(
  defineProps<WindowTitlebarProps>(),
  {
    controlsOrder: "system",
  }
)

const { controlsOrder, windowControlsProps } = props;
const osType = ref<OsType | undefined>(undefined)

onMounted(() => {
  getOsType().then((type) => {
    osType.value = type
  })
})

const left = computed(() =>
  controlsOrder === "left" ||
  (controlsOrder === "platform" && windowControlsProps?.platform === "macos") ||
  (controlsOrder === "system" && osType.value === "macos"))

const customProps = (ml: string) => {
  if (windowControlsProps?.justify !== undefined) return windowControlsProps

  const {
    justify: windowControlsJustify,
    className: windowControlsClassName,
    ...restProps
  } = windowControlsProps || {}
  return {
    justify: false,
    className: twMerge(windowControlsClassName, ml),
    ...restProps,
  } as WindowControlsProps
}
</script>

<template>
  <div
    :class="
      twMerge(
        'flex select-none flex-row overflow-hidden',
        $attrs.class as string
      )
    "
    data-tauri-drag-region
  >
    <template v-if="left">
      <WindowControls :="customProps('ml-0')" />
      <slot />
    </template>
    <template v-else>
      <slot />
      <WindowControls :="customProps('ml-auto')" />
    </template>
  </div>
</template>
