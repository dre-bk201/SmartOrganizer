<script setup lang="ts">
import EditIcon from "~icons/edit";
import TrashIcon from "~icons/trash";

import { useModel, reactive, watchEffect } from "vue";
import { dialog, invoke } from "@tauri-apps/api";

interface Props {
  modelValue: IListener;
}

interface DirCount {
  files: number;
  folders: number;
}

const props = defineProps<Props>();

let proxy = useModel(props, "modelValue");
let counts = reactive<DirCount[]>([]);

async function edit(idx: number) {
  let path = await dialog.open({ directory: true, multiple: false });
  if (path) proxy.value.monitors[idx] = path as string;
}

function remove(idx: number) {
  proxy.value.monitors.splice(idx, 1);
}

watchEffect(() => {
  proxy.value.monitors.forEach((path, idx) => {
    invoke("get_dir_count", { path }).then((count) => {
      if (idx <= counts.length) counts[idx] = count as DirCount;
      else counts.push(count as DirCount);
    });
  });
});
</script>
<template>
  <li
    v-for="(path, idx) in modelValue.monitors"
    :key="idx"
    :style="{ '--i': idx }"
    class="gap-3 py-3 dark:shadow-xl shadow-md dark:shadow-[#222] px-3 rounded-lg items-center flex adaptive--dark"
  >
    <!-- FOLDER SEARCH ICON -->
    <img class="w-[70px]" src="../../assets/img/monitor-path-icon.png" />
    <!-- FOLDER SEARCH ICON -->

    <!-- MONITOR DETAIL -->
    <div
      class="flex flex-col gap-1 flex-grow justify-center overflow-hidden dark:text-gray-300"
    >
      <p class="overflow-hidden text-ellipsis whitespace-nowrap">{{ path }}</p>
      <p class="overflow-hidden text-ellipsis whitespace-nowrap">
        <span
          class="text-blue-600 text-sm adaptive--dark--invert rounded-md text-center inline-block w-6 h-6"
          >{{ counts[idx]?.files }}</span
        >
        files |
        <span
          class="text-blue-600 text-sm adaptive--dark--invert rounded-md text-center inline-block w-6 h-6"
          >{{ counts[idx]?.folders }}</span
        >
        folders
      </p>
    </div>
    <!-- MONITOR DETAIL -->

    <div class="flex gap-3">
      <EditIcon
        @click="edit(idx)"
        class="scale-110 text-xl p-2 rounded-md box-content hover:dark:bg-dark_bg dark:text-gray-300 hover:dark:text-blue-500"
      />
      <TrashIcon
        @click="remove(idx)"
        class="scale-110 text-xl p-2 rounded-md box-content hover:dark:bg-dark_bg dark:text-gray-300 hover:dark:text-red-600"
      />
    </div>
  </li>
</template>

<style scoped>
.slide-in-move {
  transition: opacity 0.5s linear, transform 0.5s ease-in-out;
}

.slide-in-leave-active {
  transition: opacity 0.4s linear, transform 0.4s cubic-bezier(0.5, 0, 0.7, 0.4);
  transition-delay: calc(0.1s * (var(--total) - var(--i)));
}

.slide-in-enter-active {
  transition: opacity 0.5s linear, transform 0.5s cubic-bezier(0.2, 0.5, 0.1, 1);
  transition-delay: calc(0.1s * var(--i));
}

.slide-in-enter,
.slide-in-leave-to {
  opacity: 0;
}

.slide-in-enter {
  transform: translateX(-20px);
}

.slide-in-leave-to {
  transform: translateX(10px);
}
</style>
