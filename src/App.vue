<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import Titlebar from "./components/Titlebar.vue";
import Settings from "./components/Settings.vue";

import { onBeforeMount, provide, ref, computed, Ref } from "vue";
import { useStore } from "vuex";
import { Store } from "tauri-plugin-store-api";
import { listen } from "@tauri-apps/api/event";
import { Log } from "./interfaces/store/listener";
import { useToast, POSITION, TYPE } from "vue-toastification";
import anime from "animejs";
import { invoke } from "@tauri-apps/api";

// TODO Add colors to tailwind.config.css and remove static colors from elements
// TODO settings screen
// TODO a chip is pressed opens directory if valid open [Journal Component] and if not valid error popup
// TODO ellipses overflow if rule text overflows
// TODO undo button [Journal Component]
// TODO reactive status icon
// TODO Saves theming
// TODO ListenerModal Animation
// TODO Add 2 more months on chart if possible and refactor [Statistics Component]

// Effects, Classes, Constants
const store = useStore();
const toast = useToast();
const tauriStore = new Store(".data");
const configStore = new Store(".config");

const observer = new MutationObserver((mutations) => {
  for (const mut of mutations)
    store.dispatch(
      "config/setTheme",
      (<HTMLDivElement>mut.target).classList.contains("dark")
    );
});

// Provide/ Global Context
provide(
  "logDetail",
  computed(() => logDetail.value)
);

// Variables
let logDetail: Ref<Log | undefined> = ref();

// Computed
const isDark = computed(() => store.getters["config/isDark"]);

// Methods
const loadState = async () => {
  const loadedListenerData = await tauriStore.values();

  if (loadedListenerData.length)
    store.dispatch("listener/setState", {
      listeners: loadedListenerData,
    });
};

const loadConfig = async () => {
  const loadedConfig = await configStore.values();

  if (loadedConfig.length)
    store.dispatch("config/setState", {
      ...loadedConfig,
    });
};

const onLeave = (el: Element, done: () => void) =>
  anime({
    targets: el,
    opacity: 0,
    duration: 300,
    easing: "linear",
  }).finished.then(() => {
    done();
  });

const onEnter = (el: Element, done: () => void) => {
  const leftBlock = el.querySelector(".left--block");
  const rightBlock = el.querySelector(".right--block");

  anime({
    targets: leftBlock,
    translateY: ["-100%", "0px"],
    duration: 500,
  }).finished.then(() => {
    anime({
      targets: [leftBlock, rightBlock],
      opacity: [1, 0],
      duration: 100,
      easing: "linear",
    }).finished.then(() => {
      (<HTMLDivElement>leftBlock).style.transform = "translateY(-100%)";
      (<HTMLDivElement>rightBlock).style.transform = "translateY(100%)";
      done();
    });
  });

  anime({
    targets: rightBlock,
    translateY: ["100%", "0px"],
    duration: 500,
  });
};

// LifeCycle
onBeforeMount(async () => {
  // watches class attribute of #app to control theming
  const app = document.getElementById("app");

  observer.observe(app!, {
    attributes: true,
    attributeOldValue: true,
    attributeFilter: ["class"],
  });

  if (isDark.value) {
    if (!app?.classList.contains("dark")) app?.classList.add("dark");
  } else app?.classList.remove("dark");

  // Starts listening for fsevents
  invoke("start_receiver");

  // Listens for logs and change the store
  listen("logger", (log: any) => {
    logDetail.value = <Log>log.payload;

    store.dispatch("triggerClean", true);
    store.dispatch("listener/addListenerLog", log.payload);

    setTimeout(() => {
      logDetail.value = undefined;
      store.dispatch("triggerClean", false);
    }, 2500);
  });

  listen("actionFailure", (response: any) => {
    toast(response.payload.message, {
      position: POSITION.BOTTOM_RIGHT,
      type: TYPE.ERROR,
    });
    console.log("Failure Response: ", response);
  });

  await loadConfig();
  await loadState();
});
</script>

<template>
  <div class="main--layout w-screen h-screen flex rounded-lg overflow-hidden bg-l_secondary dark:bg-d_secondary">
    <Transition @enter="onEnter" @leave="onLeave">
      <Settings />
    </Transition>
    <Navbar />

    <div class="w-full flex flex-col">
      <Titlebar />
      <router-view v-slot="{ Component }">
        <keep-alive include="Dashboard">
          <component :is="Component" />
        </keep-alive>
      </router-view>
    </div>
  </div>
</template>
