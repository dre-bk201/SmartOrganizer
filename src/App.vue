<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import Titlebar from "./components/Titlebar.vue";

import { onMounted, onBeforeMount, provide, ref, computed, Ref } from "vue";
import { useStore } from "vuex";
import { Store } from "tauri-plugin-store-api";
import { listen } from "@tauri-apps/api/event";
import { Log } from "./store/modules/listener";

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
const tauriStore = new Store(".data");

const observer = new MutationObserver((mutations) => {
  for (const mut of mutations)
    isDark.value = (<HTMLDivElement>mut.target).classList.contains("dark");
});

// Variables
let isDark = ref(true);
let logDetail: Ref<Log | undefined> = ref();

// Methods
const loadState = async () => {
  let listenerDatas = await tauriStore.values();

  if (listenerDatas.length)
    store.dispatch("listener/setState", {
      listeners: listenerDatas,
    });
};

// Provide/ Global Context
provide(
  "isDark",
  computed(() => isDark.value)
);

provide(
  "logDetail",
  computed(() => logDetail.value)
);

// Hooks
onBeforeMount(async () => {
  // watches class attribute of #app to control theming
  observer.observe(document.getElementById("app")!, {
    attributes: true,
    attributeOldValue: true,
    attributeFilter: ["class"],
  });

  await loadState();
});

onMounted(() => {
  listen("logger", (log: any) => {
    logDetail.value = <Log>log.payload;

    store.dispatch("triggerClean", true);
    store.dispatch("listener/addLog", log.payload);

    setTimeout(() => {
      logDetail.value = undefined;
      store.dispatch("triggerClean", false);
    }, 2500);
  });
});
</script>

<template>
  <div
    class="main--layout w-screen h-screen flex rounded-lg overflow-hidden bg-l_secondary dark:bg-d_secondary"
  >
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
