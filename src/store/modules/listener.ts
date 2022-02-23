import { invoke } from "@tauri-apps/api/tauri";
import { Store } from "tauri-plugin-store-api";
import { v4 } from "uuid";

const store = new Store(".data");

export interface Rule {
  search_type: string;
  condition: string;
  text: string;
}

export interface Log {
  id: string;
  path: string;
  destination: string;
  action: string;
  timestamp: string;
}

export type Action = [string, string];

export interface Listener {
  id: string;
  deep: boolean;
  title: string;
  enabled: boolean;
  paths: Array<string>;
  selection: string;
  rules: Array<Rule>;
  actions: Array<Action>;
  logs: Array<Log>;
}

export interface State {
  listeners: Array<Listener>;
}

export const initialListener: Listener = {
  id: v4(),
  deep: false,
  enabled: false,
  title: "",
  paths: ["C:/Test Workspace/Github"],
  selection: "Any of the following",
  rules: [],
  actions: [],
  logs: [],
};

export const namespaced = true;

export const state = {
  listeners: [],
};

export const getters = {
  getListeners(_state: State) {
    return state.listeners;
  },

  getNumOfLogs(state: State) {
    let total = state.listeners.reduce((prev, curr) => {
      return prev + curr.logs.length;
    }, 0);

    return total;
    // state.listeners.forEach(listener => {
    //   listener.
    // })
  },
  last(state: State) {
    let len = state.listeners.length - 1;
    let logs_len = state.listeners[len].logs.length - 1;
    return state.listeners[len].logs[logs_len];
  },
};
export const mutations = {
  addListener({ listeners }: State, payload: Listener) {
    listeners.push(payload);
    store.set(payload.id, payload);
    store.save();
  },

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
    console.log("Removing ", listener[0]);
    invoke("delete_listener", { listener: listener[0] });
    store.delete(listener[0].id);
    store.save();
  },

  setState(state: State, loaded_state: State) {
    console.log("Setting Mutation state");
    console.log(loaded_state);
    state.listeners = loaded_state.listeners;
  },

  addLog(state: State, log: Log) {
    let idx = (state.listeners as Listener[]).findIndex((item) => {
      console.log(log.id, item.id);
      if (log.id == item.id) return true;
      return false;
    });
    console.log(idx);
    console.log("Log: ", log);

    if (idx != -1) {
      state.listeners[idx].logs.push(log);
      console.log(state.listeners[idx]);
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
    console.log("Setting Action state");
    commit("setState", loaded_state);
  },

  addLog({ commit, dispatch }: any, log: Log) {
    console.log("Log: ", log);
    commit("addLog", log);
    dispatch("modal/addLog", log, { root: true });
  },
};
