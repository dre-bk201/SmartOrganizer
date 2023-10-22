<script setup lang="ts">
import TrashIcon from "~icons/trash";

import { useModel, computed } from "vue";
import { ALLOWED } from "../../static";
import { dialog } from "@tauri-apps/api";

const props = defineProps<{ modelValue: IListener }>();

const proxy = useModel(props, "modelValue");
const staticFormData = ["MOVE", "COPY", "DELETE", "UNLINK", "RENAME"];


let selected = computed(() => {
  return proxy.value.actions.map(({ action }) => action);
});

function isAllowed(idx: number, action: string) {
  if (proxy.value.actions.length == 1) return true;

  return selected.value.reduce((acc, curr) =>
    acc && ALLOWED[curr].includes(action)
    || proxy.value.actions[idx].action == action
    , true);
}

async function onActionChange(data: string, idx: number) {
  if (data == "DELETE") {
    proxy.value.actions[idx].action = data as TActionOpts;
    return;
  }

  let path = await dialog.open({ multiple: false, directory: true });
  proxy.value.actions[idx] = { action: data as TActionOpts, path: path as string }
}

function remove(idx: number) {
  proxy.value.actions.splice(idx, 1);
}
</script>

<template>
  <li :key="idx" v-for="({ action, path }, idx) in modelValue.actions"
    class="gap-3 py-2 list-none shadow-xl px-3 rounded-lg adaptive--dark dark:text-gray-200 text-gray-800">
    <span class="flex-grow whitespace-nowrap text-xs overflow-ellipsis">
      Please select actions to perform: {{ path }}
    </span>
    <hr class="border-gray-400"/>

    <div class="flex mt-1 justify-between items-center">
      <form class="grid grid-cols-3 mt-1 gap-y-1 gap-x-5">
        <label v-for="data in staticFormData"
          :class="[!isAllowed(idx, data) && 'opacity-40', 'transition-all flex gap-2 items-center']">
          <input name="actions" type="radio" @input="onActionChange(data, idx)" :value="data" :checked="data == action"
            :disabled="!isAllowed(idx, data)" />
          {{ data }}
        </label>
      </form>

      <div class="flex gap-2 items-center">
        <TrashIcon @click="remove(idx)"
          class="adaptive--dark--invert hover:text-red-500 text-xl p-2 box-content rounded-lg" />
      </div>
    </div>
  </li>
</template>
