<script lang="ts" setup>
import Icon from "../components/Icon.vue";

import { open } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api";
import { computed, ref } from "vue";
import { useStore } from "vuex";

interface Props {
  parent_id: string;
  id: string;
  idx: number;
  path: string;
  action: string;
  timestamp: string;
  destination: string;
}

const props = defineProps<Props>();
const store = useStore();

let detail = ref("");
let title = store.getters["listener/getById"](props.parent_id)?.title;

const setInfo = (value: string = "") => (detail.value = value);

const unsupported = computed(
  () => !["DELETE", "UNLINK"].includes(props.action)
);

const undoAction = async () => {
  const response = await invoke("undo_action", {
    id: props.id,
    from: props.path,
    action: [props.action, props.destination],
  });

  console.log(response);
};

const openDir = async (path: string) => await open(path);
</script>

<template>
  <div class="log bg-l_white dark:bg-d_secondary py-2 rounded-md relative mb-5">
    <Transition name="slide-fade">
      <div
        v-if="detail"
        class="info flex bg-l_primary dark:bg-d_primary absolute right-20 px-2 rounded-md"
      >
        {{ detail }}
      </div>
    </Transition>
    <section class="flex w-full px-4 pr-2 mb-1">
      <div class="icon--bg rounded-full p-2 bg-l_primary mr-5">
        <Icon
          :name="action?.toLowerCase()"
          width="30px"
          height="30px"
          fill="black"
        />
      </div>
      <div class="grow">
        <header>{{ title }}</header>
        <span>
          Updated At:
          {{
            new Date(timestamp).toLocaleString("en-US", {
              year: "numeric",
              month: "numeric",
              day: "numeric",
              hour: "numeric",
              minute: "numeric",
              second: "numeric",
            })
          }}
        </span>
      </div>
      <button
        v-show="unsupported"
        @click="undoAction"
        class="bg-[#CBCBCB] dark:bg-d_primary px-2 h-8 rounded-md"
      >
        UNDO
      </button>
    </section>

    <section class="grid-view md:grid-view-md px-2">
      <header class="ml-5">Monitoring</header>
      <header class="ml-2">Action</header>
      <header class="ml-5">Destination</header>
      <header class="ml-2">Timestamp</header>

      <!-- Monitor Chip -->
      <span
        @mouseover="setInfo(path)"
        @click="openDir(path)"
        @mouseout="setInfo('')"
        class="log--card max-w-fit text-[#FF5C5C] relative bg-[#FF5C5C]"
        >{{ path }}
        <div class="dot bg-[#FF5C5C]" />
      </span>

      <!-- Action Chip -->
      <span
        @mouseover="setInfo(action)"
        @mouseout="setInfo('')"
        class="rounded-full text-center hover:cursor-pointer relative text-[#00F0FF] bg-[#00E9F8] bg-opacity-30 min-w-fit max-w-fit pl-4 px-3"
        >{{ action }}
        <div class="dot bg-[#00F0FF]" />
      </span>

      <!-- Destination Chip -->
      <span
        @mouseover="setInfo(destination)"
        @click="openDir(destination)"
        @mouseout="setInfo('')"
        class="log--card max-w-fit text-[#4200FE] bg-[#4200FE] dark:bg-[#EBFF6F] dark:text-[#EBFF6F] dark:bg-opacity-30"
        >{{ destination ? destination : "No Directory Required For Operation" }}
        <div class="dot bg-[#4200FE] dark:bg-[#EBFF6F]" />
      </span>

      <!-- Timestamp Chip -->
      <span
        @mouseover="setInfo(timestamp)"
        @mouseout="setInfo('')"
        class="log--card text-[#FF00A8] max-w-fit bg-[#FF00A8]"
        >{{ timestamp }}
        <div class="dot bg-[#FF00A8]" />
      </span>
    </section>
  </div>
</template>

<style scoped>
/* .slide-enter-active {
  transform: translateY(17px);
}
.slide-leave-active {
}
.slide-enter-from,
.slide-leave-to {
} */
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}
.slide-fade-leave-active {
  transition: all 0.3s cubic-bezier(1, 0.5, 0.8, 1);
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}

button {
  box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.25);
}
</style>
