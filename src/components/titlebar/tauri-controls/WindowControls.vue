<script setup lang="ts">
import { twMerge } from "tailwind-merge"
import { onMounted, ref } from "vue"
import Gnome from "./controls/linux/Gnome.vue"
import MacOs from "./controls/MacOs.vue"
import Windows from "./controls/Windows.vue"
import type { WindowControlsProps } from "./types"
import { getOsType } from "./utils/os"

defineOptions({
  inheritAttrs: false,
})

const props = withDefaults(defineProps<WindowControlsProps>(), {
  justify: false,
  hide: false,
  hideMethod: "display",
  className: "",
})

const platform = ref(props.platform)
onMounted(() => {
  getOsType().then((type) => {
    if (!platform.value) {
      switch (type) {
        case "macos":
          platform.value = "macos"
          break
        case "linux":
          platform.value = "gnome"
          break
        default:
          platform.value = "windows"
      }
    }
  })
})

const customClass = twMerge(
  "flex",
  props.className,
  props.hide && (props.hideMethod === "display" ? "hidden" : "invisible")
)
</script>

<template>
  <Windows
    v-if="platform === 'windows'"
    :class="twMerge(customClass, props.justify && 'ml-auto')"
  />
  <MacOs
    v-else-if="platform === 'macos'"
    :class="twMerge(customClass, props.justify && 'ml-0')"
  />
  <Gnome
    v-else-if="platform === 'gnome'"
    :class="twMerge(customClass, props.justify && 'ml-auto')"
  />
  <Windows v-else :class="twMerge(customClass, props.justify && 'ml-auto')" />
</template>
