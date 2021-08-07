import { createStore } from "vuex";
import { invoke } from "@tauri-apps/api";
// TODO
// > Remove Listener *
// > Fix Sidepane ram issue
// > Add logs support from rust
// > Add Empty states
// > Add Dialog for creating an action_paths
// > Figure out statistics

const store = createStore({
  state: {
    sidepaneData: {},
    modalData: {},
    trash: {},
    listeners: [
      // {
      //   search: "mkv txt mp4",
      //   deep: false,
      //   index: 0,
      //   logs: [
      //     {
      //       type_: "COPY",
      //       rule: "",
      //       index: 0,
      //       timestamp: "2021-08-04T00:36:25.306Z",
      //       monitor: "",
      //       destination: "",
      //     },
      //   ],
      //   title: "Anime Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      // },
      // {
      //   search: "rar gz tar xz",
      //   deep: false,
      //   index: 1,
      //   logs: [],
      //   title: "Compression Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      // },
      // {
      //   search: "mkv txt mp4",
      //   deep: false,
      //   index: 2,
      //   logs: [],
      //   title: "Media Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      // },
      // {
      //   search: "rar gz tar xz",
      //   deep: false,
      //   index: 3,
      //   logs: [],
      //   title: "Free Fire Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      // },
    ],
  },
  getters: {
    getSidepaneData: (state) => {
      return state.sidepaneData;
    },
    getModalData: (state) => {
      return state.modalData;
    },
    getTrash: (state) => {
      return state.trash;
    },
    getListeners: (state) => {
      return state.listeners;
    },
  },

  mutations: {
    updateModalData: (state, data) => {
      state.modalData = data;
    },

    updateListener(state, listener) {
      state.listeners[listener.index] = listener;
    },

    removeListener(state, listener) {
      state.listeners.splice(listener.index, 1);
      invoke("remove_listener", { index: listener.index });
      state.listeners.forEach((_, index, arr) => {
        arr[index].index = index;
      });
    },

    addListener: (state, listener) => {
      state.listeners.push(listener);
    },

    updateSidepaneData: (state, data) => {
      state.sidepaneData = data;
    },

    updateTrash: (state, listener) => {
      state.trash = listener;
    },

    updateLogs: (state, log) => {
      state;
      console.log("State Logs: ", log);
      let updatedLog = Object.assign({}, log);
      updatedLog.index = state.listeners[log.index].logs.length;
      state.listeners[log.index].logs.push(updatedLog);
    },
  },
});

invoke("load_state").then((data) => {
  data.forEach((listener) => {
    console.log("Loaded State Listener: ", listener);
    store.state.listeners.push(listener);
  });
});

store.watch(
  (state) => state.listeners,
  (val) => {
    setTimeout(() => {
      invoke("save_state", { data: store.state.listeners });
      console.log("Saving the state: ", val);
    });
  },
  { deep: true }
);

export default store;
