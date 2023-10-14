//@ts-nocheck
import { defineStore } from "pinia";
import { InjectionKey } from "vue";

import { Listeners, Logs } from "../data";
import { invoke } from "@tauri-apps/api";

export const LISTENERSTORE = "listenerStore";

export const useListenerStore = defineStore(LISTENERSTORE, {
  state: () => ({
    search: "",
    activeId: "",
    logs: [] as ILog[],
    listeners: [] as IListener[],
  }),

  getters: {
    activeListener(state) {
      let listener = state.listeners.find((lst) => lst.id == state.activeId);
      return listener;
    },

    getListeners(state) {
      return state.listeners;
    },
  },

  actions: {
    async addLog(log: ILog) {
      let isSaved = await invoke("save_log", { log });
      if (isSaved) this.logs.push(log);
      else console.log("Unable to save log: ", isSaved);
    },

    setActiveId(id: string = "") {
      this.activeId = id;
    },

    async deleteListener(id: string) {
      const deleted = await invoke<null | IListener>("remove_listener", {
        listener: this.findById(id),
      });
      const idx = this.findIdxById(deleted.id);
      if (idx > -1) this.listeners.splice(idx, 1);
    },

    toggleEnabled(id: string) {
      let found = this.findById(id);
      if (found) this.updateById(id, { ...found, enabled: !found.enabled });
    },

    async updateById(id: string, state: IListener) {
      await invoke<string | null>("update_listener", {
        listener: state,
      });
      let idx = this.findIdxById(id);
      if (idx > -1) this.listeners[idx] = state;
    },

    async setListenerState(listeners: IListener[]) {
      this.listeners = listeners;
    },

    async setLogState(logs: ILog[]) {
      this.logs = logs;
    },

    async addListener(listener: IListener) {
      let id = await invoke("add_listener", { listener });
      if (id == listener.id) this.listeners.push(listener);
    },

    findIdxById(id: string) {
      return this.listeners.findIndex((lst) => lst.id == id);
    },

    findById(id: string) {
      return this.listeners.find((lst) => lst.id == id);
    },

    setSearch(v: string) {
      this.search = v;
    },
  },
});

interface ModalOpts {
  interactOutside: boolean,
  escapeKeyDown: boolean
}

export type TListenerStore = ReturnType<typeof useListenerStore>;
export const ListenerModalKey = Symbol() as InjectionKey<{
  open: (id: string) => void,
  close: () => void,
  remove: (id: string) => void,
  modalOpts: ModalOpts
}>;
export const ListenerStoreKey = Symbol() as InjectionKey<{
  listener: TListenerStore;
}>;
