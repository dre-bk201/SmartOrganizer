import { invoke } from "@tauri-apps/api/tauri";
import { Store } from "tauri-plugin-store-api";
import { v4 } from "uuid";
import { Listener, State, Log } from "../../interfaces/store/listener";

const store = new Store(".data");

export const initialListener: Listener = {
  id: v4(),
  deep: false,
  enabled: false,
  title: "",
  paths: [],
  selection: "Any",
  rules: [],
  actions: [],
  logs: [],
};

export const namespaced = true;

export const state = {
  listeners: [],
};

export const getters = {
  listeners(state: State) {
    return state.listeners;
  },

  // refactor: Change from using `find` to `findIndex`
  getById: (state: State) => (id: string) => {
    return state.listeners.find((listener) => listener.id === id);
  },

  allLogs(state: State) {
    let initialValue: Log[] = [];
    return state.listeners.reduce(
      (acc, val) => acc.concat(...val.logs),
      initialValue
    );
  },

  getNumOfLogs(state: State) {
    let total = state.listeners.reduce((prev, curr) => {
      return prev + curr.logs.length;
    }, 0);

    return total;
  },
};

export const mutations = {
  addListener({ listeners }: State, payload: Listener) {
    listeners.push(payload);
    store.set(payload.id, payload);
    store.save();
  },

  // updateDeep({listener}:)

  updateListener(state: State, payload: Listener) {
    let idx = state.listeners.findIndex((item) => {
      if (item.id == payload.id) return true;
      return false;
    });

    if (idx != -1) {
      state.listeners[idx] = payload;
      invoke("update_listener", { listener: payload });
      store.set(payload.id, payload);
      store.save();
    }
  },

  deleteListener(state: State, id: string) {
    let idx = state.listeners.findIndex((item) => {
      if (item.id == id) return true;
      return false;
    });

    let listener = state.listeners.splice(idx, 1);
    invoke("delete_listener", { listener: listener[0] });
    store.delete(listener[0].id);
    store.save();
  },

  setState(state: State, loaded_state: State) {
    state.listeners = loaded_state.listeners;
  },

  addLog(state: State, log: Log) {
    let idx = (state.listeners as Listener[]).findIndex((item) => {
      if (log.parent_id == item.id) return true;
      return false;
    });

    if (idx != -1) {
      state.listeners[idx].logs.push(log);
      store.set(state.listeners[idx].id, state.listeners[idx]);
      store.save();
    }
  },
};

export const actions = {
  addListener({ commit, dispatch }: any, payload: Listener) {
    commit("addListener", payload);
    dispatch("modal/setListener", JSON.parse(JSON.stringify(payload)), {
      root: true,
    });
  },

  updateListener({ commit }: any, payload: Listener) {
    commit("updateListener", payload);
  },

  deleteListener({ commit }: any, id: string) {
    commit("deleteListener", id);
  },

  setState({ commit }: any, loaded_state: State) {
    commit("setState", loaded_state);
  },

  addLog({ commit, dispatch }: any, log: Log) {
    commit("addLog", log);
    dispatch("modal/addLog", log, { root: true });
  },

  getById({ commit }: any, id: string) {
    commit("getById", id);
  },
};
