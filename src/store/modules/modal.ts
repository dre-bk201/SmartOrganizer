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
  toggleActive(_state: State) {
    let c = _state.listener as Listener;
    _state.listener = {
      ..._state.listener,
      enabled: !(_state.listener as Listener).enabled,
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

  addRule(state: State, rule: Rule) {
    (state.listener as Listener).rules.push(rule);
  },

  setSelection(state: State, payload: SelectionType) {
    (state.listener as Listener).selection = payload;
  },

  setAction(state: State, [action, idx, path]: [string, number, string]) {
    (state.listener as Listener).actions[idx][0] = action;
    (state.listener as Listener).actions[idx][1] = path;
  },

  createAction(state: State) {
    (state.listener as Listener).actions.push(["", ""]);
  },

  addLog(state: State, log: Log) {
    (state.listener as Listener).logs.push(log);
  },

  removeAction: (state: State, idx: number) =>
    (state.listener as Listener).actions.splice(idx, 1),
};

export const actions = {
  toggleActive({ commit }: any) {
    commit("toggleActive");
  },

  setListener({ commit }: any, payload: Listener) {
    commit("setListener", payload);
  },

  closeListenerDetail({ commit }: any) {
    commit("closeListenerDetail");
  },

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

  setAction({ commit }: any, payload: [string, number, string]) {
    commit("setAction", payload);
  },

  async addRule({ commit }: any) {
    commit("addRule", {
      search_type: "",
      condition: "",
      text: "",
    });
  },

  setSelection({ commit }: any, payload: SelectionType) {
    commit("setSelection", payload);
  },

  createAction({ commit }: any) {
    commit("createAction");
  },

  addLog({ commit, state }: any, log: Log) {
    if (((state as State).listener as Listener).id == log.id)
      commit("addLog", log);
  },

  removeAction: ({ commit }: any, idx: number) => {
    commit("removeAction", idx);
  },
};
