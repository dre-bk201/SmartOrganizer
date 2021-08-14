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
      //       monitor: "/home/h4ck3r/Sandbox",
      //       destination: "/home/h4ck3r/Downloads/Compressed",
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
      //   title: "Compression Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      //   logs: [
      //     {
      //       type_: "COPY",
      //       index: 0,
      //       monitor:
      //         "/home/h4ck3r/Downloads/Video/Watch Tsukimichi -Moonlit Fantasy Episode 1- English Subbed .mp4",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Aug 2021 22:03:07 -0500",
      //     },
      //     {
      //       type_: "COPY",
      //       index: 1,
      //       monitor:
      //         "/home/h4ck3r/Downloads/Video/Watch Tsukimichi -Moonlit Fantasy Episode 2- English Subbed .mp4",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Aug 2021 22:03:08 -0500",
      //     },
      //     {
      //       type_: "COPY",
      //       index: 2,
      //       monitor:
      //         "/home/h4ck3r/Downloads/Video/Watch Tsukimichi -Moonlit Fantasy Episode 3- English Subbed .mp4",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Jan 2021 22:03:10 -0500",
      //     },
      //     {
      //       type_: "COPY",
      //       index: 3,
      //       monitor:
      //         "/home/h4ck3r/Downloads/Video/Watch Tsukimichi -Moonlit Fantasy Episode 4- English Subbed .mp4",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Oct 2021 22:03:10 -0500",
      //     },
      //     {
      //       type_: "COPY",
      //       index: 4,
      //       monitor:
      //         "/home/h4ck3r/Downloads/Video/Watch Tsukimichi -Moonlit Fantasy- English Subbed Online Free.mp4",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Jun 2021 22:03:11 -0500",
      //     },
      //     {
      //       type_: "COPY",
      //       index: 5,
      //       monitor: "/home/h4ck3r/Downloads/Video/demo1.m4v",
      //       destination: "/home/h4ck3r/Downloads/",
      //       timestamp: "Sat, 07 Jan 2021 22:03:10 -0500",
      //     },
      //   ],
      // },
      // {
      //   search: "mkv txt mp4",
      //   deep: false,
      //   index: 2,
      //   title: "Media Listener",
      //   enable_smart_organizer: false,
      //   type_: "FileExtension",
      //   rule: "Includes",
      //   monitor_paths: ["/home/h4ck3r/Pictures/Winter/"],
      //   action_paths: { "/home/h4ck3r/Sandbox/": ["COPY"] },
      //   logs: [
      //     {
      //       type_: "MOVE",
      //       index: 0,
      //       monitor: "/home/h4ck3r/Sandbox/Compressed/Bryan.zip",
      //       destination: "/home/h4ck3r/Sandbox/",
      //       timestamp: "Sat, 07 Mar 2021 22:03:10 -0500",
      //     },
      //     {
      //       type_: "MOVE",
      //       index: 1,
      //       monitor: "/home/h4ck3r/Sandbox/Compressed/Comfortaa.zip",
      //       destination: "/home/h4ck3r/Sandbox/",
      //       timestamp: "Sat, 07 Aug 2021 21:40:40 -0500",
      //     },
      //     {
      //       type_: "MOVE",
      //       index: 2,
      //       monitor: "/home/h4ck3r/Sandbox/Compressed/Hall.zip",
      //       destination: "/home/h4ck3r/Sandbox/",
      //       timestamp: "Sat, 07 Aug 2021 21:40:40 -0500",
      //     },
      //     {
      //       type_: "MOVE",
      //       index: 3,
      //       monitor: "/home/h4ck3r/Sandbox/Compressed/Keys.zip",
      //       destination: "/home/h4ck3r/Sandbox/",
      //       timestamp: "Sat, 07 Jan 2021 22:03:10 -0500",
      //     },
      //     {
      //       type_: "MOVE",
      //       index: 4,
      //       monitor:
      //         "/home/h4ck3r/Sandbox/Compressed/mcOS-BigSur-Large.layout.latte.zip",
      //       destination: "/home/h4ck3r/Sandbox/",
      //       timestamp: "Sat, 07 Feb 2021 21:40:40 -0500",
      //     },
      //   ],
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
    getTotalLogsCount: (state) => {
      let total = 0;
      const listeners = state.listeners;

      for (let i = 0; i < listeners.length; i++)
        total += listeners[i].logs.length;

      return total;
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
