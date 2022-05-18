<script lang="ts" setup>
// TODO implement tree browser
// TODO give animation
// TODO only be able to set active if the right conditions are met (Has a title, has a path if move,copy, rename, etc..)

import Icon from "./Icon.vue";
import Router from "./RouterLink.vue";
import Switch from "./Switch.vue";

import { useStore } from "vuex";
import { useRoute, useRouter } from "vue-router";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { dialog } from "@tauri-apps/api";
import { Listener } from "../interfaces/store/listener";
import { normalize } from "@tauri-apps/api/path";

const store = useStore();
const route = useRoute();
const router = useRouter();

// Variables
let enableTextField = ref(false);
let inputField = ref<HTMLInputElement | null>();

// Computed
const modalState = computed(() => store.state.modal.listener);
const isDark = computed(() => store.getters["config/isDark"]);

// Functions
const closeDetailListener = () => store.dispatch("modal/closeListenerDetail");
const saveOptions = () =>
  store.dispatch(
    "modal/saveOptions",
    JSON.parse(JSON.stringify(modalState.value))
  );

const edit = (val: boolean = false) => {
  enableTextField.value = val;
  setTimeout(() => inputField.value?.focus(), 0);
};

const updateTitle = (e: Event) => {
  store.dispatch("modal/updateTitle", (<HTMLInputElement>e.target).value);
};

const deleteListener = () => {
  let idx = JSON.parse(JSON.stringify(modalState.value as Listener)).id;
  closeDetailListener();

  store.dispatch("listener/deleteListener", idx);
};

const floatingButtonAction = async () => {
  switch (route.name) {
    case "Monitor":
      let path = await dialog.open({ directory: true, multiple: false });

      if (path) {
        let normalized_path = await normalize(path as string);
        store.dispatch("modal/addMonitorPath", normalized_path);
      }
      break;
    case "Rules":
      let idx = store.state.modal.listener.rules.length;
      store.dispatch("modal/addListenerRule");
      store.dispatch("modal/setCurrentRule", {
        idx,
        searchType: "",
        condition: "",
        text: "",
      });
      break;
    case "Action":
      store.dispatch("modal/addListenerAction");
      break;
  }
};

// Lifecycle Methods
onMounted(() => {
  router.push({ path: "/monitor" });
});

onUnmounted(() => {
  router.replace({ path: "/" });
});
</script>

<template>
  <div
    class="listener--detail overflow-hidden rounded-lg bg-l_primary dark:bg-d_primary dark:text-gray-300 h-full w-full absolute flex left-0 top-0"
  >
    <div
      :class="`statusbar h-full w-1  ${
        modalState.enabled ? 'bg-[#61FF5E]' : 'bg-l_secondary'
      }`"
    ></div>
    <div
      data-tauri-drag-region
      class="detail p-3 bg-l_white dark:bg-d_secondary opacity-65 min-w-[240px] max-w-[240px] h-full relative"
    >
      <!-- Show button if len of getListeners -->
      <button @click="deleteListener" class="delete">Delete Listener</button>

      <div class="content flex flex-col items-center pt-14">
        <div class="floating--button top-16 right-[calc(-30px)]">
          <button
            @click="floatingButtonAction"
            class="bg-[#6C8DFF] p-2 rounded-full"
          >
            <Icon name="plus" width="25" height="25" fill="white" />
          </button>
        </div>

        <Icon name="folder" width="120" height="120" fill="#A7BAFA" />

        <div
          class="content--text mt-7 mb-2 p-2 flex flex-col items-center justify-between"
        >
          <div class="title flex items-center">
            <Icon
              @click="edit(true)"
              class="inline-block hover:cursor-pointer"
              name="edit"
              :key="(isDark as number)"
              :fill="isDark ? '#b6b6b6' : 'black'"
              width="20"
              height="20"
            />

            <input
              v-show="enableTextField"
              ref="inputField"
              placeholder="Listener Title"
              class="text-xl w-[90%] indent-2 outline-none rounded-md bg-l_primary dark:focus:bg-d_primary"
              type="text"
              :value="modalState.title"
              @focusout="edit(false)"
              @input="updateTitle"
            />
            <input
              v-show="!enableTextField"
              placeholder="Listener Title"
              disabled
              class="text-xl w-[90%] indent-2 bg-transparent"
              :value="modalState.title"
            />
            <!-- <span class="text-xl font">{{ modalState.title }}</span> -->
          </div>
          <span class="detail--1 text-md font-light">
            Has been organized {{ modalState.logs?.length }} times
          </span>
          <span class="detail--2 text-md font-light"
            >Contains {{ modalState.paths?.length }} listeners</span
          >
          <div>
            <input
              id="deep"
              class="mt-4"
              type="checkbox"
              v-model="modalState.deep"
            />
            <label for="deep" class="ml-2 mb-4">Deep Search</label>
          </div>
        </div>

        <a class="mb-5 text-[#6C8DFF] hover:cursor-pointer" @click.prevent=""
          >View Tree Browser</a
        >

        <Switch />

        <div class="navigation absolute bottom-4 w-16 flex justify-between">
          <Router to="/monitor" />
          <Router to="/rules" />
          <Router to="/actions" />
        </div>
      </div>
    </div>

    <div class="page h-full flex-1 flex flex-col">
      <div data-tauri-drag-region class="bar h-0 relative py-3">
        <!-- Buttons container -->
        <div class="absolute right-5 bottom-[-30px]">
          <button
            @click="closeDetailListener"
            class="bg-[#F3F3F3] dark:bg-d_secondary px-3 py-1 z-[9999999] mr-2 rounded-md shadow-lg hover:shadow-none"
          >
            Close
          </button>
          <button
            @click="saveOptions"
            class="bg-[#FF3B76] px-3 py-1 ml-2 rounded-md shadow-lg hover:shadow-none text-white"
          >
            Save
          </button>
        </div>
      </div>
      <router-view v-slot="{ Component }">
        <component :is="Component"></component>
      </router-view>
    </div>
  </div>
</template>
