<script setup lang="ts">
import CloseIcon from "~icons/close";

import { ref, useModel } from 'vue';

const props = defineProps<{ modelValue: string[] }>();
const inputRef = ref<HTMLInputElement | null>(null);

let chip = ref("")
let proxy = useModel(props, "modelValue");

function removeChip(idx: number) {
  proxy.value.splice(idx, 1);
}

function handleKeyPress(key: KeyboardEvent) {
  if (key.key === "Enter" && chip.value.length) {
    proxy.value.push((key.target as HTMLInputElement).value);
    chip.value = ""
  } else if (key.key == "Backspace" && !chip.value.length) {
    key.preventDefault();
    proxy.value.pop();
  }
}
</script>

<template>
  <div @click="inputRef?.focus()" class="bg-[#424349] box-border outline-primary rounded-md pl-2 py-[10px] text-xs">
    <ul class="flex gap-2 items-center">
      <li class="flex items-center py-1 gap-1 dark:bg-darker rounded-md px-2 box-content "
        v-for="text, idx in modelValue">
        <CloseIcon @click="removeChip(idx)" class="text-xs bg-white text-red-400 rounded-full" />
        {{ text }}
      </li>
      <input ref="inputRef" @keydown="handleKeyPress" :style="{ width: chip.length + 2 + 'ch' }"
        class="outline-none h-6 rounded-sm bg-transparent" type="text" v-model="chip" />
    </ul>
  </div>
</template>

