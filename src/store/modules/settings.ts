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

export const mutations = {};

export const actions = {};
