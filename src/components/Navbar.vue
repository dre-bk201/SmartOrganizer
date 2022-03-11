<script setup lang="ts">
import Icon from "./Icon.vue";
import Navlink from "./Navlink.vue";
import Broom from "./Broom.vue";

import { computed, ref } from "vue";
import { useStore } from "vuex";
import { useRoute } from "vue-router";
import { v4 } from "uuid";
import { initialListener } from "../store/modules/listener";

// Hooks
const store = useStore();
const route = useRoute();

let isHovering = ref(false);
let hoveringTimeout = ref<ReturnType<typeof setTimeout>>();

// Computed
const isValidHovering = computed(
  () => isHovering.value && route.name != "Statistics"
);

// Functions
const handleHoverEnter = (e: MouseEvent) => {
  if (e.target) {
    hoveringTimeout.value = setTimeout(() => (isHovering.value = true), 500);
  }
};

const handleHoverExit = (e: MouseEvent) => {
  if (e.target && hoveringTimeout.value) {
    clearTimeout(hoveringTimeout.value);
    isHovering.value = false;
  }
};

const addListener = () => {
  store.dispatch("listener/addListener", { ...initialListener, id: v4() });
};

const updateList = (e: Event) => {
  if (route.name == "Dashboard")
    store.dispatch(
      "updateDashboardSearch",
      (<HTMLInputElement>e.target)?.value
    );
  else if (route.name == "Journal")
    store.dispatch("updateLogSearch", (<HTMLInputElement>e.target)?.value);
};
</script>

<template>
  <div
    data-tauri-drag-region
    @mouseover.self="handleHoverEnter"
    @mouseleave="handleHoverExit"
    :class="`nav ${
      isValidHovering ? 'show' : ''
    } h-full relative bg-l_secondary dark:bg-d_secondary`"
  >
    <!-- Title -->
    <div class="title mt-5 mb-10 flex justify-center text-white text-xl">
      <!-- Broom Icon -->
      <Broom />
      <h1
        :class="`${
          isValidHovering ? 'opacity-100 w-fit' : 'opacity-0 w-0'
        } h-6 pt-2`"
      >
        Smart Organizer
      </h1>
    </div>

    <!-- Search -->
    <div class="search mt-5 px-3 flex justify-center relative items-center">
      <!-- Search Icon -->
      <Icon
        :class="`absolute ${isValidHovering ? 'left-4' : 'left-[11px]'}`"
        name="search"
        :key="Number(isValidHovering)"
        :width="isValidHovering ? '15' : '33'"
        :height="isValidHovering ? '15' : '33'"
      />

      <!-- Search Field -->
      <input
        :class="`${
          isHovering && route.name != 'Statistics'
            ? 'opacity-100'
            : 'opacity-0 flex justify-center'
        } rounded-md bg-l_primary dark:bg-d_primary dark:text-gray-300 outline-none w-full indent-6 h-7 text-sm`"
        @input="updateList"
        type="text"
        placeholder="Seach here..."
      />
    </div>

    <!-- Links Container -->
    <div
      class="links--container flex flex-col mt-10 items-center justify-evenly"
    >
      <!-- Dashboard Link -->
      <Navlink
        name="Dashboard"
        path="/"
        icon="dashboard"
        :isHovering="isHovering && route.name != 'Statistics'"
      />

      <!-- Journal Link -->
      <Navlink
        name="Journal"
        path="/journal"
        icon="journal"
        :isHovering="isHovering && route.name != 'Statistics'"
      />

      <!-- Statistics Link -->
      <Navlink
        name="Statistics"
        path="/statistics"
        icon="statistics"
        :isHovering="isHovering && route.name != 'Statistics'"
      />
    </div>

    <!-- Floating Action Button -->
    <div class="floating--button bottom-24 left-[calc(100%-1.75rem)]">
      <button @click="addListener" class="bg-[#6C8DFF] p-2 rounded-full">
        <Icon name="plus" width="25" height="25" fill="white" />
      </button>
    </div>
  </div>
</template>
