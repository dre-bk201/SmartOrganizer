interface State {
  isDark: boolean;
  titlebar: "win32" | "macos";
  pinNavbar: "show" | "hide" | "hover";
  scanningInterval: number;
  chunks: number;
}

export const namespaced = true;

export const state: State = {
  isDark: false,
  titlebar: "macos",
  pinNavbar: "hover",
  scanningInterval: 5000,
  chunks: 30,
};

export const getters = {
  titlebar: (state: State) => state.titlebar,
  isDark: (state: State) => state.isDark,
  scanningInterval: (state: State) => state.scanningInterval,
  pinNavbar: (state: State) => state.pinNavbar,
  chunks: (state: State) => state.chunks,
};

export const mutations = {};

export const actions = {};
