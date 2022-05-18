<script lang="ts" setup>
import Icon from "./Icon.vue";
import type {
  SelectionType,
  Listener,
  Rule,
  Action,
  Log,
  ListenerData,
} from "../interfaces/store/listener";

import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from "vuex";

import successIcon from "../assets/icons/success.png";
import warningIcon from "../assets/icons/warning.svg";

interface Props extends Listener {
  id: string;
  deep: boolean;
  enabled: boolean;
  title: string;
  paths: string[];
  selection: SelectionType;
  rules: Array<Rule>;
  actions: Action[];
  logs: Log[];
}

const props = defineProps<Props>();
const store = useStore();

let isBusy = ref(false);
let listenerInterval = ref<ReturnType<typeof setInterval>>();

const end = (arr: Array<any>): any | undefined => {
  // returns the last element
  if (arr.length) return arr[arr.length - 1];
  return undefined;
};

// Computed

const scanningInterval = computed(
  () => store.getters["config/scanningInterval"]
);

const isToday = computed(() => {
  if (end(props.logs) != undefined) {
    let today = new Date().setHours(0, 0, 0, 0);
    let timestamp = new Date(end(props.logs).timestamp).setHours(0, 0, 0, 0);

    return today == timestamp;
  }
  return false;
});

const isYesterday = computed(() => {
  if (end(props.logs) != undefined) {
    let yesterday = new Date();
    let timestamp = new Date(end(props.logs).timestamp);

    yesterday.setDate(new Date().getDate() - 1);

    return yesterday.setHours(0, 0, 0, 0) == timestamp.setHours(0, 0, 0, 0);
  }
  return false;
});

const formatDate = computed(() => {
  let prefix = isToday.value ? "Today" : isYesterday.value ? "Yesterday" : "";

  if (!props.logs.length) return "-/-/-";

  let timestamp: Date = new Date(end(props.logs).timestamp);
  switch (prefix) {
    case "Today":
      return `${prefix} ${timestamp.toLocaleString("en-US", {
        hour: "numeric",
        minute: "numeric",
      })}`;

    case "Yesterday":
      return `${prefix} ${timestamp.toLocaleString("en-US", {
        hour: "numeric",
        minute: "numeric",
      })}`;

    default:
      return `${prefix} ${timestamp.toLocaleString("en-US", {
        year: "numeric",
        month: "numeric",
        day: "numeric",
      })}`;
  }
});

const showListenerDetail = (e: MouseEvent) => {
  store.dispatch("modal/setListener", JSON.parse(JSON.stringify(props)));
};

// Lifecycle Methods
onMounted(() => {
  let listener: ListenerData = {
    id: props.id,
    deep: props.deep,
    enabled: props.enabled,
    paths: props.paths,
    selection: props.selection,
    rules: props.rules,
    actions: props.actions,
  };

  invoke("add_listener", { listener });
});
</script>

<template>
  <div
    @click="showListenerDetail"
    class="listener w-full min-h-[5.4rem] rounded-md relative hover:cursor-pointer bg-l_white dark:bg-d_secondary dark:text-gray-300 mb-8"
  >
    <div
      class="status--msg absolute right-3 text-xs bg-l_white dark:bg-d_secondary px-2 rounded-tl-md rounded-tr-md top-[-16.6px]"
    >
      {{ !enabled ? "Listener Not enabled" : "" }}
    </div>

    <div
      :class="`status--bar ${enabled ? 'bg-[#61FF5E]' : 'bg-l_secondary'}`"
    />

    <div class="content h-full flex items-center pr-5">
      <Icon class="ml-4" name="folder" fill="#6C8DFF" width="65" height="65" />

      <div
        class="content-text flex-1 flex flex-col h-[80%] justify-around text-2xl pl-6"
      >
        <h2 class="title-xyz">
          {{ title }}
        </h2>
        <h4 class="text-sm">Has been organized {{ logs?.length }} times</h4>
      </div>

      <div class="last-update flex items-center flex-col text-[0.65rem]">
        <img
          :src="logs.length && enabled ? successIcon : warningIcon"
          width="40"
          height="40"
        />
        <span>Last Updated {{ isToday || isYesterday ? "" : "At" }}</span>
        <span>{{ formatDate }}</span>
      </div>
    </div>
    <div
      class="listener-amt absolute right-[-8px] w-6 h-6 bottom-[-8px] shadow-md shadow-[#00000040] center text-xs text-l_white bg-[#6C8DFF] rounded-full p-2"
    >
      {{ paths.length }}
    </div>
  </div>
</template>
