<script setup lang="ts">
import Icon from "./Icon.vue";
import Navlink from "./Navlink.vue";
import Broom from "./Broom.vue";

import { ref } from "vue";
import { useStore } from "vuex";
import { useRoute } from "vue-router";
import { v4 } from "uuid";
import { Listener, initialListener } from "../store/modules/listener";

// Hooks
const store = useStore();
const route = useRoute();

let isHovering = ref(false);
let hoveringTimeout = ref<ReturnType<typeof setTimeout>>();

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
  store.dispatch("listener/addListener", initialListener);
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
      isHovering && route.name != 'Statistics' ? 'show' : ''
    } h-full relative bg-l_secondary dark:bg-d_secondary`"
  >
    <!-- Title -->
    <div class="title mt-5 mb-10 flex justify-center text-white text-xl">
      <Broom />
      <h1
        :class="`${
          isHovering && route.name != 'Statistics'
            ? 'opacity-100 w-fit'
            : 'opacity-0 w-0'
        } h-6 pt-2`"
      >
        Smart Organizer
      </h1>
    </div>

    <!-- Search -->
    <div class="search mt-5 px-3 flex justify-center relative items-center">
      <Icon
        v-show="isHovering && route.name != 'Statistics'"
        class="absolute left-4"
        name="search"
        width="15"
        height="15"
      />
      <Icon
        v-show="!(isHovering && route.name != 'Statistics')"
        class="absolute left-[11px]"
        name="search"
        width="30"
        height="30"
      />

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
    <!-- 
    <button @click="toggleTheme" class="bg-white dark:bg-slate-500">
      toggle theme
    </button> -->

    <!-- Floating Action Button -->
    <div class="floating--button bottom-24 left-[calc(100%-1.75rem)]">
      <button @click="addListener" class="bg-[#6C8DFF] p-2 rounded-full">
        <Icon name="plus" width="25" height="25" fill="white" />
      </button>
    </div>
  </div>
</template>
