import {
  Listener,
  Log,
  Rule,
  SelectionType,
} from "../../interfaces/store/listener";

interface State {
  listener: Listener | {};
  currentRule: {} | (Rule & { idx: number });
}

export const namespaced = true;

export const state: State = {
  listener: {},
  currentRule: {},
};

export const mutations = {
  toggleActive(state: State) {
    state.listener = {
      ...state.listener,
      enabled: !(state.listener as Listener).enabled,
    };
  },

  setListener(state: State, payload: Listener) {
    state.listener = payload;
  },

  closeListenerDetail(state: State) {
    state.listener = {};
  },

  updateTitle(state: State, payload: string) {
    (state.listener as Listener).title = payload;
  },

  removeMonitorPath(state: State, idx: number) {
    (state.listener as Listener).paths.splice(idx, 1);
  },

  addMonitorPath(state: State, path: string) {
    (state.listener as Listener).paths.push(path);
  },

  removeRule(state: State, idx: number) {
    (state.listener as Listener).rules.splice(idx, 1);
  },

  setCurrentRule(state: State, payload: Rule & { idx: number }) {
    state.currentRule = payload;
  },

  updateRuleByIdx(state: State, payload: Rule & { idx: number }) {
    const { idx, ...rule } = payload;
    (state.listener as Listener).rules[payload.idx] = rule;
  },

  addListenerRule(state: State, rule: Rule) {
    (state.listener as Listener).rules.push(rule);
  },

  setListenerSelectionType(state: State, payload: SelectionType) {
    (state.listener as Listener).selection = payload;
  },

  setListenerAction(
    state: State,
    [action, idx, path]: [string, number, string]
  ) {
    (state.listener as Listener).actions[idx][0] = action;
    (state.listener as Listener).actions[idx][1] = path;
  },

  addListenerAction(state: State) {
    (state.listener as Listener).actions.push(["", ""]);
  },

  addListenerLog(state: State, log: Log) {
    (state.listener as Listener).logs.push(log);
  },

  removeListenerAction: (state: State, idx: number) =>
    (state.listener as Listener).actions.splice(idx, 1),
};

export const actions = {
  toggleActive({ commit }: any) {
    commit("toggleActive");
  },

  // Sets Modal listener to whichever listener is selected
  setListener({ commit }: any, payload: Listener) {
    commit("setListener", payload);
  },

  closeListenerDetail({ commit }: any) {
    commit("closeListenerDetail");
  },

  // Closes `ListenerDetail` and updates the correct `Listener`
  saveOptions({ dispatch, commit }: any, payload: Listener) {
    dispatch("listener/updateListener", payload, { root: true });
    commit("closeListenerDetail");
  },

  updateTitle({ commit }: any, payload: string) {
    commit("updateTitle", payload);
  },

  removeMonitorPath({ commit }: any, idx: number) {
    commit("removeMonitorPath", idx);
  },

  addMonitorPath({ commit }: any, path: string) {
    commit("addMonitorPath", path);
  },

  removeRule({ commit }: any, idx: number) {
    commit("removeRule", idx);
  },

  setCurrentRule({ commit }: any, payload: Rule & { idx: number }) {
    commit("setCurrentRule", payload);
  },

  updateRuleByIdx({ commit }: any, payload: Rule & { idx: number }) {
    commit("updateRuleByIdx", payload);
  },

  setListenerAction({ commit }: any, payload: [string, number, string]) {
    commit("setListenerAction", payload);
  },

  async addListenerRule({ commit }: any) {
    commit("addListenerRule", {
      searchType: "",
      condition: "",
      text: "",
    });
  },

  setListenerSelectionType({ commit }: any, payload: SelectionType) {
    commit("setListenerSelectionType", payload);
  },

  addListenerAction({ commit }: any) {
    commit("addListenerAction");
  },

  addListenerLog({ commit, state }: any, log: Log) {
    if (((state as State).listener as Listener).id == log.id)
      commit("addListenerLog", log);
  },

  removeListenerAction: ({ commit }: any, idx: number) => {
    commit("removeListenerAction", idx);
  },
};
