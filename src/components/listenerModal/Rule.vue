<script setup lang="ts">
//@ts-nocheck
import TrashIcon from "~icons/trash";

import Dropdown from '../common/Dropdown.vue';
import Chips from "./Chips.vue";

import { conditionOpts, searchOpts, sizeOpts } from '../../static';
import { useModel } from 'vue';
import { isNumber } from "chart.js/helpers";

const props = defineProps<{ modelValue: IListener }>();

let proxy = useModel(props, "modelValue");

function update(value: string | [number, TUnitOpts], idx: number, kind: string) {
  if (kind == "search")
    proxy.value.rules[idx].search = value as TSearch;
  else if (kind == "condition")
    proxy.value.rules[idx].condition = value as TCondition;
  else if (kind == "data.size") {
    proxy.value.rules[idx].data.size = value as [number, TUnitOpts]
  }
}

function remove(idx: number) {
  proxy.value.rules.splice(idx, 1);
}
function disabled(search: TSearch) {
  let copts: TCondition[] = ["Includes", "NotIncludes", "ExactMatch", "IsNot"];
  let fsize = ["Less", "Greater", "IsEqual"];

  if(search == "FileSize") return copts;
  else return fsize
}
</script>
<template>
  <div v-for="{ search, condition, data: { text, size } }, idx in proxy.rules"
    class="flex gap-3 items-center adaptive--dark px-3 py-2 rounded-lg">
    <span class="text-xl">If</span>

    <Dropdown :model-value="search" @update:model-value="(e) => update(e, idx, 'search')" :options="searchOpts" />

    <Dropdown :disabled="{state:search, value:disabled(search)}" :model-value="condition" @update:model-value="(e) => update(e, idx, 'condition')"
      :options="conditionOpts" />

    <Chips v-show="search != 'FileSize'" class="flex-grow w-0 overflow-x-auto" :model-value="text" />

    <div class="flex gap-4 flex-grow items-center" v-show="search == 'FileSize'">
      <input class="bg-[#424349] py-2 flex-grow outline-none rounded-md pr-2 text-lg text-right" type="number"
        :value="size[0]" @input="(e) => update([+(e.target as HTMLInputElement).value, size[1]], idx, 'data.size')" />

      <Dropdown :model-value="size[1]" @update:model-value="(e) => update([size[0], e], idx, 'data.size')"
        :options="sizeOpts" />
    </div>

    <TrashIcon @click="remove(idx)"
      class="ml-2 flex-shrink-0 box-content p-2 text-lg adaptive--dark--invert rounded-md" />
  </div>
</template>

<style scoped>
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
    /* display: none; <- Crashes Chrome on hover */
    -webkit-appearance: none;
    margin: 0; /* <-- Apparently some margin are still there even though it's hidden */
}

input[type=number] {
    -moz-appearance:textfield; /* Firefox */
}
</style>
