import { createStore } from "vuex";
import * as listener from "./modules/listener";
import * as modal from "./modules/modal";
import * as config from "./modules/config";

const store = createStore({
  modules: {
    listener,
    modal,
    config,
  },
  state: {
    dashboardSearch: "",
    logSearch: "",
    triggerClean: false,
    isSettingsOpen: false,
  },
  getters: {
    triggerClean: (state) => state.triggerClean,
    dashboardSearch: (state) => state.dashboardSearch,
    logSearch: (state) => state.logSearch,
    isSettingsOpen: (state) => state.isSettingsOpen,
  },

  mutations: {
    updateDashboardSearch: (state, payload: string) =>
      (state.dashboardSearch = payload),
    triggerClean: (state, payload: boolean) => (state.triggerClean = payload),
    updateLogSearch: (state, payload: string) => (state.logSearch = payload),
    toggleSettings: (state, payload: boolean) =>
      (state.isSettingsOpen = payload),
  },
  actions: {
    setCurrentPage({ commit }, payload: string) {
      commit("setCurrentPage", payload);
    },

    updateDashboardSearch({ commit }, payload: string) {
      commit("updateDashboardSearch", payload);
    },

    triggerClean({ commit }, payload: boolean) {
      commit("triggerClean", payload);
    },

    setListenerRect({ commit }, payload: DOMRect) {
      commit("setListenerRect", payload);
    },

    updateLogSearch({ commit }, payload: string) {
      commit("updateLogSearch", payload);
    },

    toggleSettings: ({ commit }, payload: boolean) =>
      commit("toggleSettings", payload),
  },
});

export default store;
