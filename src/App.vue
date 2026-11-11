<script setup lang="ts">
import AppLayout from "./components/layout/AppLayout.vue";
import Titlebar from "./components/titlebar/Titlebar.vue";
import Navbar from "./components/navbar/Navbar.vue";
import ListenerModal from "./components/listenerModal/ListenerModal.vue";
import {
  Dialog,
  DialogContent,
  DialogTrigger,
  DialogTitle,
} from '@/components/ui/dialog'
import { DialogClose } from "radix-vue";

import {
  useSettingStore,
  useListenerStore,
  ListenerModalKey,
  ListenerStoreKey,
  SettingStoreKey,
  ThemeKey,
} from "./store";

import { reactive, nextTick, computed, provide, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { platform } from "@tauri-apps/plugin-os";

import { confirm } from '@tauri-apps/plugin-dialog';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process'

const settingStore = useSettingStore();
const listenerStore = useListenerStore();

const appRoot = ref<HTMLElement | null>(null);
const modal = ref<InstanceType<typeof ListenerModal> | null>(null);

let triggerClean = ref(false);
let modalOpts = reactive({
  interactOutside: false,
  escapeKeyDown: false
});

function escapeKeyDown(e: KeyboardEvent) {
  if (!modalOpts.escapeKeyDown) { e.preventDefault(); void modal.value?.requestClose(); }
}

function interactOutside(e: Event) {
  if (!modalOpts.interactOutside) e.preventDefault();
}

provide(SettingStoreKey, {
  settings: settingStore,
});

provide(ThemeKey, {
  isDark: computed<boolean>(() => settingStore.isDark),
  theme: computed(() => settingStore.theme),
});

provide(ListenerStoreKey, {
  listener: listenerStore,
});

const open = (id: string) => {
  listenerStore.setActiveId(id);
  (appRoot.value?.querySelector(".modalTriggerOpen")! as HTMLButtonElement)?.click();
};

const close = () => {
  listenerStore.setActiveId();
  (appRoot.value?.querySelector(".modalTriggerClose")! as HTMLButtonElement)?.click();
};

const remove = (id: string) => {
  open(id);
  nextTick(() => modal.value?.remove());
}

provide(ListenerModalKey, {
  open,
  close,
  remove,
  modalOpts
});

provide("triggerCleaning", triggerClean);

onMounted(() => {
  (async () => {
    let settings = await invoke("load_settings");
    if (settings) {
      settingStore.setState(settings);
      if (settingStore.theme === 'dark') document.documentElement.classList.add('dark');
      else document.documentElement.classList.remove('dark');
    }

    let { logs, listeners }: { logs: ILog[], listeners: IListener[] } = await invoke("load_from_database");

    await listenerStore.setListenerState(listeners);
    await listenerStore.setLogState(logs);

    listen("log", (a) => {
      listenerStore.addLog(a.payload as ILog);
      triggerClean.value = true;
    });

    // Starts listener filesystem service
    await invoke("start_listener");


    // UPDATER
    try {
      const update = await check();
      if (update && await confirm(`SmartOrganizer ${update.version} is available. Install and restart now?`, { title: "Update available", kind: "info" })) {
        await update.downloadAndInstall();
        if (platform() !== "windows") await relaunch();
      }
    } catch (error) {
      console.error("Update check failed", error);
    }

  })();


});
</script>

<template>
  <div class="relative" ref="appRoot">
    <AppLayout class="rounded-md" :setting-store="settingStore">
      <template #listenerModal="{ radius }">
        <Dialog>
          <DialogTrigger as="button" class="modalTriggerOpen" />
          <DialogClose class="modalTriggerClose" />
          <DialogContent @interact-outside="interactOutside" @escape-key-down="escapeKeyDown"
            class="listener-dialog">
            <DialogTitle class="sr-only">Edit listener</DialogTitle>
            <ListenerModal ref="modal" :open="open" :close="close" :listener="listenerStore" :radius="radius" />
          </DialogContent>
        </Dialog>
      </template>

      <template #titlebar>
        <Titlebar :setting-store="settingStore" />
      </template>

      <template #navbar="{ adjustWidth }">
        <Navbar @adjust-width="adjustWidth" :setting-store="settingStore" />
      </template>

      <template #default>
        <router-view></router-view>
      </template>
    </AppLayout>
  </div>
</template>

<style scoped></style>
