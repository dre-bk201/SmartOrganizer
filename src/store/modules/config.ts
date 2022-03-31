import { Store } from "tauri-plugin-store-api";

const config = new Store(".config");

const loadState: Array<[string, unknown]> = await config.entries();

const initialState: State = loadState.reduce(
  (v, item) => Object.assign(v, { [item[0]]: item[1] }),
  {
    isDark: false,
    titlebar: "macos",
    pinNavbar: "pin",
    scanningInterval: 5000,
    chunks: 30,
  }
);

type OS = "win32" | "macos" | "auto";

interface State {
  isDark: boolean;
  titlebar: OS;
  pinNavbar: "pin" | "unpin";
  scanningInterval: number;
  chunks: number;
}

export const namespaced = true;

export const state: State = {
  ...initialState,
};

export const getters = {
  titlebar: (state: State) => state.titlebar,
  isDark: (state: State) => state.isDark,
  scanningInterval: (state: State) => state.scanningInterval,
  pinNavbar: (state: State) => state.pinNavbar,
  chunks: (state: State) => state.chunks,
  config: (state: State) => state,
};

export const mutations = {
  setTheme: (state: State, theme: boolean) => {
    state.isDark = theme;
    config.set("isDark", theme);
    config.save();
  },
  setState: (state: State, loadedState: State) => {
    state = loadedState;
    console.log("Setting state: ", loadedState);
  },
  setTitlebar: (state: State, titlebar: OS) => {
    state.titlebar = titlebar;
    config.set("titlebar", titlebar);
    config.save();
  },
  setChunks: (state: State, chunk: number) => {
    state.chunks = chunk;
    config.set("chunks", chunk);
    config.save();
  },
  setInterval: (state: State, interval: number) => {
    state.scanningInterval = interval;
    config.set("scanningInterval", interval);
    config.save();
  },
  setPinNavbar: (state: State, option: "pin" | "unpin") => {
    state.pinNavbar = option;
    config.set("pinNavbar", option);
    config.save();
  },
};

export const actions = {
  setTheme: ({ commit }: any, theme: boolean) => commit("setTheme", theme),

  setState: ({ commit }: any, loaded_state: State) =>
    commit("setState", loaded_state),

  setTitlebar: ({ commit }: any, titlebar: OS) =>
    commit("setTitlebar", titlebar),

  setChunks: ({ commit }: any, chunk: number) => commit("setChunks", chunk),

  setInterval: ({ commit }: any, interval: number) =>
    commit("setInterval", interval),

  setPinNavbar: ({ commit }: any, option: "pin" | "unpin") =>
    commit("setPinNavbar", option),
};
