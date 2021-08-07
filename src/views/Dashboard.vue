<template>
  <div id="Dashboard">
    <div class="body" @click.stop="triggerCloseEvent">
      <teleport to="#modal">
        <!-- <keep-alive> -->
        <ListenerModal v-if="Object.keys(getModalData).length" />
        <!-- </keep-alive> -->
      </teleport>
      <!-- <ListenerModal v-show="Object.keys(getModalData).length" /> -->
      <div class="content-container">
        <!-- <div v-show="!getListeners.length" class="empty-state">
          <img
            draggable="false"
            src="../assets/images/idle-empty-state.png"
            alt=""
          />
          Click "+" to add listeners
        </div> -->
        <template v-for="(listener, index) in getListeners" :key="index">
          <Listener :listener="listener" :index="index" />
        </template>
      </div>
      <i class="plus" @click.stop="showModal" v-html="plus"></i>
      <Trash v-if="Object.keys(getTrash).length" />
    </div>
    <div class="h-spacer"></div>
    <Sidepane node="" />
  </div>
</template>

<script>
// @ is an alias to /src
import Sidepane from "@/components/Dashboard/Sidepane";
import Listener from "@/components/Dashboard/Listener";
import ListenerModal from "@/components/Dashboard/ListenerModal";
import Trash from "@/components/Dashboard/Trash.vue";
import { invoke } from "@tauri-apps/api";

import { plus } from "@/icons";

import { mapGetters } from "vuex";
export default {
  name: "Dashboard",
  components: {
    Sidepane,
    Trash,
    Listener,
    ListenerModal,
  },
  data: () => ({
    plus,
  }),
  computed: {
    ...mapGetters(["getModalData", "getListeners", "getTrash"]),
  },
  methods: {
    showModal() {
      this.$store.commit("updateModalData", {
        type_: "", // *
        index: null,
        rule: "", // *
        title: "", // *
        deep: false,
        logs: [],
        search: "", // *
        action_paths: {},
        monitor_paths: [], // *
        enable_smart_organizer: false,
      });
    },
    filterLogs(logs) {
      logs.forEach((logArr) => {
        logArr.forEach((log) => {
          if (log.type_) {
            // console.log("Logs: ", log);
            this.$store.commit("updateLogs", log);
          }
        });
      });
    },

    triggerCloseEvent() {
      this.$store.commit("updateSidepaneData", {});
    },
  },

  mounted() {
    invoke;
    const sendSignal = async () => {
      if (this.getListeners.length) {
        let logs = await invoke("run_organizer");
        this.filterLogs(logs);
      }
    };
    sendSignal();
    setInterval(async () => {
      // if (this.getListeners.length) {
      //   let logs = await invoke("run_organizer");
      //   this.filterLogs(logs);
      // }
      sendSignal();
    }, 5000);
  },
};
</script>

<style lang="scss" scoped>
#Dashboard {
  @include flexAlignCenter(row);
  border-radius: 30px 0px 0px;
  background: $darkbg;
  height: 100%;
  width: calc(100vw - #{$nav-width});
  position: absolute;
  top: 0px;
  overflow: hidden;
  left: $nav-width;

  .body {
    border-top-left-radius: 15px;
    transition: 0.4s;
    position: relative;
    flex-grow: 1;
    height: 100%;
    display: flex;
    width: 100%;

    .content-container {
      position: absolute;
      @include fullDimension();
      @include flexAlignCenter(column);
      padding: 50px 20px 10px 20px;
      box-sizing: border-box;
      // height: 100%;
      // width: 100%;
      // display: flex;
      // flex-flow: column;
      // align-items: center;

      // #Trash {
      //   // position: absolute;
      // }
    }

    .plus {
      @include flexAlignCenter(row);
      background: $primary;
      position: absolute;
      border-radius: 10px;
      transition: 0.3s;
      fill: white;
      padding: 5px;
      bottom: 15px;
      right: 15px;
      width: 30px;
      height: 30px;
      // z-index: 1;
      box-shadow: $boxshadow;
    }

    .plus:hover {
      box-shadow: 0px 0px 0px 0px;
      cursor: pointer;
    }
  }
  .h-spacer {
    height: 100%;
    min-width: $padding;
    min-height: $padding;
    background: $darkprimary;
  }
}
</style>
