<script setup lang="ts">
import AppLayout from "./components/layout/AppLayout.vue";
import Titlebar from "./components/titlebar/Titlebar.vue";
import Navbar from "./components/navbar/Navbar.vue";
import ListenerModal from "./components/listenerModal/ListenerModal.vue";
import {
  Dialog,
  DialogContent,
  DialogTrigger,
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
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { FocusOutsideEvent, PointerDownOutsideEvent } from "node_modules/radix-vue/dist/DismissableLayer";

import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

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
  if (!modalOpts.escapeKeyDown) e.preventDefault();
}

function interactOutside(e: PointerDownOutsideEvent | FocusOutsideEvent) {
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
    if (settings) settingStore.setState(settings);

    let { logs, listeners }: { logs: ILog[], listeners: IListener[] } = await invoke("load_from_database");

    await listenerStore.setListenerState(listeners);
    await listenerStore.setLogState(logs);

    listen("log", (a) => {
      listenerStore.addLog(a.payload as ILog);
      triggerClean.value = true;
    });

    await invoke("start_listener");

    if (settingStore.theme === 'dark') document.documentElement.classList.add('dark');
    else document.documentElement.classList.remove('dark');


    const unlisten = await onUpdaterEvent(({ error, status }) => {
      // This will log all updater events, including status updates and errors.
      console.log('Updater event', error, status)
    })

    try {
      const { shouldUpdate, manifest } = await checkUpdate()

      if (shouldUpdate) {
        // You could show a dialog asking the user if they want to install the update here.
        console.log(
          `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
        )

        // Install the update. This will also restart the app on Windows!
        await installUpdate()

        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        await relaunch()
      }
    } catch (error) {
      console.error(error)
    }

    // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
    unlisten()


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
            class="p-0 max-w-[100vw] mt-[calc(17px)] border-none h-[calc(100vh-34px)] outline-none">
            <ListenerModal ref="modal" :open="open" :close="close" :listener="listenerStore" :radius="radius" />
          </DialogContent>
        </Dialog>
      </template>

      <template #titlebar>
        <Titlebar :setting-store="settingStore" />
      </template>

      <template #navbar="{ adjustWidth }">
        <Navbar class="adaptive--darker" @adjust-width="adjustWidth" :setting-store="settingStore" />
      </template>

      <template #default>
        <router-view></router-view>
      </template>
    </AppLayout>
  </div>
</template>

<style scoped></style>
