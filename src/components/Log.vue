<script lang="ts" setup>
import { Listener, Log } from "../store/modules/listener";

import Icon from "../components/Icon.vue";
import { ref } from "vue";

interface Props {
  listener: Listener;
  log: Log;
}

defineProps<Props>();

let detail = ref("");

const setInfo = (value: string = "") => {
  detail.value = value;
};

const undoAction = () => {
  // invoke a method to tauri to reverse command
};
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
          :name="log.action.toLowerCase()"
          width="30px"
          height="30px"
          fill="black"
        />
      </div>
      <div class="grow">
        <header>{{ listener.title }}</header>
        <span> Last updated: {{ log.timestamp }} </span>
      </div>
      <button
        @click="undoAction"
        class="bg-[#CBCBCB] dark:bg-d_primary px-1 h-8 rounded-md"
      >
        UNDO
      </button>
    </section>

    <section class="grid-view md:grid-view-md px-2">
      <header class="ml-5">Monitor Dir</header>
      <header class="ml-2">Action</header>
      <header class="ml-5">Destination</header>
      <header class="ml-2">Timestamp</header>

      <!-- Monitor Chip -->
      <span
        @mouseover="setInfo(log.path)"
        @mouseout="setInfo('')"
        class="log--card max-w-fit text-[#FF5C5C] relative bg-[#FF5C5C]"
        >{{ log.path }}
        <div class="dot bg-[#FF5C5C]" />
      </span>

      <!-- Action Chip -->
      <span
        @mouseover="setInfo(log.action)"
        @mouseout="setInfo('')"
        class="rounded-full text-center hover:cursor-pointer relative text-[#00F0FF] bg-[#00E9F8] bg-opacity-30 min-w-fit max-w-fit pl-4 px-3"
        >{{ log.action }}
        <div class="dot bg-[#00F0FF]" />
      </span>

      <!-- Destination Chip -->
      <span
        @mouseover="setInfo(log.destination)"
        @mouseout="setInfo('')"
        class="log--card max-w-fit text-[#4200FE] bg-[#4200FE] dark:bg-[#EBFF6F] dark:text-[#EBFF6F] dark:bg-opacity-30"
        >{{
          log.destination
            ? log.destination
            : "No Directory Required For Operation"
        }}
        <div class="dot bg-[#4200FE] dark:bg-[#EBFF6F]" />
      </span>

      <!-- Timestamp Chip -->
      <span
        @mouseover="setInfo(log.timestamp)"
        @mouseout="setInfo('')"
        class="log--card text-[#FF00A8] max-w-fit bg-[#FF00A8]"
        >{{ log.timestamp }}
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
</style>
