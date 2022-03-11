import { createStore } from "vuex";
import * as listener from "./modules/listener";
import * as modal from "./modules/modal";
import * as settings from "./modules/settings";

const store = createStore({
  modules: {
    listener,
    modal,
    settings,
  },
  state: {
    dashboardSearch: "",
    logSearch: "",
    triggerClean: false,
    listenerRect: {},
  },
  getters: {
    triggerClean: (state) => state.triggerClean,

    listenerRect: (state) => state.listenerRect,

    dashboardSearch: (state) => state.dashboardSearch,

    logSearch: (state) => state.logSearch,
  },

  mutations: {
    updateDashboardSearch: (state, payload: string) =>
      (state.dashboardSearch = payload),

    triggerClean: (state, payload: boolean) => (state.triggerClean = payload),

    setListenerRect: (state, payload: DOMRect) =>
      (state.listenerRect = payload),

    updateLogSearch: (state, payload: string) => (state.logSearch = payload),
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
  },
});

export default store;
