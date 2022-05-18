<script lang="ts" setup>
// TODO export the constant actions and import them throughout components (the inputelements) for ease of change
import DetailCard from "./DetailCard.vue";

import { ref, onMounted, ComponentPublicInstance } from "vue";
import { dialog } from "@tauri-apps/api";
import { useStore } from "vuex";

import { COPY, MOVE, DELETE, UNLINK, RENAME } from "../utils";
import { normalize } from "@tauri-apps/api/path";

const props = defineProps<{ action: [string, string]; idx: number }>();
const store = useStore();

// Variables
let rootRef = ref<ComponentPublicInstance | null>();

// Functions
const toggleInputValue = (input: HTMLInputElement) => {
  if (!input.checked) input.checked = !input.checked;
};

const selectAction = async (e: Event, idx: number) => {
  const [input, label]: any = (<HTMLInputElement>e.target).closest(
    "div"
  )?.children;

  if (input) {
    toggleInputValue(<HTMLInputElement>input);

    let action = (<HTMLInputElement>input).value;
    let path: string | string[] | null = null;

    if (props.idx == idx) {
      const storeCommit = async (
        action: string,
        warn: boolean = true,
        isFile: boolean = false
      ) => {
        if (!warn) store.commit("modal/setListenerAction", [action, idx, ""]);
        else {
          if (!isFile)
            path = await dialog.open({ directory: true, multiple: false });
          else path = await dialog.save({});

          if (path) {
            let normalized_path = await normalize(path as string);
            store.commit("modal/setListenerAction", [
              action,
              idx,
              normalized_path,
            ]);
          }
        }
      };

      switch (action) {
        case COPY:
        case MOVE:
          await storeCommit(action);
          break;
        case RENAME:
          await storeCommit(action, true, true);
          break;

        case DELETE:
        case UNLINK:
          await storeCommit(action, false);
          break;

        default:
          break;
      }
    }
  }
};

const removeListenerAction = (idx: number) =>
  store.dispatch("modal/removeListenerAction", idx);

onMounted(() => {
  let root = rootRef.value?.$el;
  let len = store.state.modal.listener.actions.length;

  for (let i = 0; i < len; i++) {
    let inputs: Array<HTMLInputElement> = root.querySelectorAll("input[name]");
    inputs.forEach((input) => {
      if (props.action[0] == input.value) {
        input.checked = true;
      }
    });
  }
});
</script>

<template>
  <DetailCard ref="rootRef" class="mx-8 text-gray-500 dark:text-gray-300">
    <template #header>
      <div class="header relative flex items-center p-[3px]">
        <span class="header pl-3 text-sm"
          >Action to perform: {{ action[1] }}</span
        >
        <button
          @click.stop="() => removeListenerAction(idx)"
          class="absolute right-6 text-xs hover:text-[#FF0303]"
        >
          Remove
        </button>
      </div>
    </template>

    <template #content>
      <div class="options--grid px-2">
        <div @click.stop="selectAction($event, idx)" class="action--radio">
          <input
            class="mr-2"
            :id="'MOVE-' + idx"
            type="radio"
            :name="'action-' + idx"
            value="MOVE"
          />

          <label class="text-lg">MOVE</label>
        </div>

        <div @click.stop="selectAction($event, idx)" class="action--radio">
          <input
            class="mr-2"
            :id="'COPY-' + idx"
            type="radio"
            :name="'action-' + idx"
            value="COPY"
          />
          <label class="text-lg">COPY</label>
        </div>

        <div @click.stop="selectAction($event, idx)" class="action--radio">
          <input
            class="mr-2"
            :id="'DELETE-' + idx"
            type="radio"
            :name="'action-' + idx"
            value="DELETE"
          />
          <label class="text-lg">DELETE</label>
        </div>

        <div @click.stop="selectAction($event, idx)" class="action--radio">
          <input
            @click.stop="selectAction($event, idx)"
            class="mr-2"
            :id="'UNLINK-' + idx"
            type="radio"
            :name="'action-' + idx"
            value="UNLINK"
          />
          <label class="text-lg">UNLINK</label>
        </div>

        <div @click.stop="selectAction($event, idx)" class="action--radio">
          <input
            class="mr-2"
            :id="'RENAME-' + idx"
            type="radio"
            :name="'action-' + idx"
            value="RENAME"
          />
          <label class="text-lg">RENAME</label>
        </div>
      </div>
    </template>
  </DetailCard>
</template>
