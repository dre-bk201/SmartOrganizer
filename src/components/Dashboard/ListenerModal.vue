<template>
  <div id="ListenerModal">
    <div class="content">
      <div class="blur-bg" @click="hideModal" />
      <div class="dialog">
        <h2 class="title">{{ activeScreen }}</h2>
        <MaterialField v-model="listenerData.title" @keypress.enter="save" />
        <div class="content" style="background: transparent">
          <div class="screen" style="background: transparent">
            <Monitor v-if="activeScreen == 'Monitor'" />
            <keep-alive>
              <Rules
                v-if="activeScreen == 'Rules'"
                v-model:rule="listenerData.rule"
                v-model:type="listenerData.type_"
                v-model="listenerData.search"
              />
            </keep-alive>
            <Action
              v-if="activeScreen == 'Action'"
              v-model:actions="listenerData.action_paths"
              :listener="listenerData"
            />
          </div>
          <div class="carousel-controller">
            <div
              class="control-btn active"
              @click="focus($event, 'Monitor')"
            ></div>
            <div class="control-btn" @click="focus($event, 'Rules')"></div>
            <div class="control-btn" @click="focus($event, 'Action')"></div>
          </div>
          <div class="footer">
            <Switch v-if="activeScreen == 'Rules'" />
            <img
              src="@/assets/images/add-folder.svg"
              v-if="activeScreen == 'Monitor' || activeScreen == 'Action'"
              @click="activeScreen == 'Monitor' ? addLocation() : addAction()"
            />
            <button @click="hideModal">Cancel</button>
            <button @click="save">Save</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import anime from "animejs";
import Monitor from "./MonitorScreen.vue";
import Action from "./ActionScreen.vue";
import Rules from "./RuleScreen.vue";
import Switch from "./Switch.vue";
import MaterialField from "./MaterialField.vue";
import { dialog } from "@tauri-apps/api";
import { computed } from "@vue/runtime-core";
import { mapGetters } from "vuex";
import { mapMutations } from "vuex";
import { reactive } from "vue";

export default {
  components: {
    MaterialField,
    Monitor,
    Action,
    Rules,
    Switch,
  },
  data: () => ({
    screens: ["Monitor", "Rules", "Action"],
    activeScreen: "Monitor",
    animation: null,
    isSaving: false,
    // tempDeleted: {
    //   title: "",
    //   index: 0,
    //   deep: true,
    //   type_: "",
    //   rule: "",
    //   search: "",
    //   enable_smart_organizer: false,
    //   monitor_paths: [],
    //   action_paths: {},
    // },
  }),
  provide() {
    const data = this.listenerData;
    return {
      remove: this.removeLocation,
      updateAction: this.updateAction,
      listenerData: computed(() => data),
    };
  },
  computed: {
    ...mapGetters(["getModalData", "getListeners"]),
  },
  methods: {
    ...mapMutations(["updateModalData", "addListener", "updateListener"]),
    // only able to close modal after finished opening
    validateData() {},
    showModal() {
      let root = this.$el;
      let blur = root.querySelector("#ListenerModal .content .blur-bg");
      let dialog = root.querySelector(".dialog");
      let animeCount = 0;

      root.classList.remove("hide");
      anime({
        targets: blur,
        backdropFilter: ["blur(0px)", "blur(2px)"],
        filter: ["blur(0px)", "blur(5px)"],
        background: "rgba(0,0,0,0.5)",
        duration: 1000,
        easing: "easeInOutSine",
        update: function (anim) {
          if (Math.trunc(anim.progress) > 50) {
            if (animeCount == 1) {
              anime({
                targets: dialog,
                width: "360px",
                easing: "linear",
                duration: 200,
              });
            }
            animeCount += 1;
          }
        },
      }).finished.then(() => console.log("Animation finished"));
    },

    hideModal() {
      let root = this.$el;
      let blur = root.querySelector("#ListenerModal .content .blur-bg");
      let dialog = root.querySelector(".dialog");
      anime({
        targets: dialog,
        width: "0px",
        easing: "linear",
        duration: 200,
      }).finished.then(() => {
        anime({
          targets: blur,
          backdropFilter: "blur(0px)",
          filter: "blur(0px)",
          background: "rgba(0,0,0,0)",
          duration: 400,
        });
      });
      // const updateModalData = this.updateModalData;
      // updateModalData({});
      // if (!this.isSaving) {
      //   console.log("Default");
      //   this.updateListener(this.tempDeleted);
      //   const data = this.tempDeleted;
      //   data.monitor_paths.forEach((item) => {
      //     data.monitor_paths.push(item);
      //   });
      // }
      setTimeout(() => {
        this.updateModalData({});
      }, 450);
    },

    async addLocation() {
      // disable save button if there is no monitor paths
      let path = await dialog.open({ directory: true });
      if (!path.endsWith("/")) path += "/"; // path always ends with '/' N.B [for rust backend]

      if (path && !this.listenerData.monitor_paths.includes(path)) {
        this.listenerData.monitor_paths.push(path);
        console.log(this.listenerData);
      }
      // this.listenerData.monitor_paths.push("asdlkad");
    },

    async addAction() {
      dialog;
      let path = await dialog.open({ directory: true });
      if (!path.endsWith("/")) path += "/";

      if (path) this.listenerData.action_paths[path] = [];
      // this.listenerData.action_paths["/home/user/Downloads/"] = ["COPY"];
    },

    updateAction(key, value) {
      this.listenerData.action_paths[key] = value;
      console.log(this.listenerData);
    },

    removeLocation(index, type) {
      const data = this.listenerData;
      if (type == "action") {
        let key = Object.keys(data.action_paths);
        // this.tempDeleted.action.push(data.action_paths[key]);
        delete data.action_paths[key];
      } else {
        this.listenerData.monitor_paths.splice(index, 1);
        // this.tempDeleted.monitor_paths.push(removedValue[0]);
      }
    },

    focus(event, screen) {
      this.activeScreen = screen;
      const element = event.target;
      document
        .querySelectorAll("#ListenerModal .carousel-controller .control-btn")
        .forEach((ele) => {
          ele.classList.remove("active");
        });
      element.classList.add("active");
    },

    save() {
      let addListener = true;
      const listenerData = this.listenerData;
      if (listenerData.index == null) {
        listenerData.index = this.getListeners.length;
      }

      this.getListeners.forEach((list) => {
        if (list.index == listenerData.index) {
          this.updateListener(listenerData);
          addListener = false;
        }
      });
      if (addListener) this.addListener(listenerData);
      this.isSaving = true;
      this.hideModal();
    },
  },

  mounted() {
    // debugger;
    // this.bus.on("updateModalData", (data) => {
    //   console.log("At mounted: ", this.listenerData);
    //   this.listenerData = data;
    // });

    // this.bus.on("updateModalData", (data) => {
    //   console.log("At mounted: ", this.listenerData);
    //   this.listenerData = data;
    //   this.showModal();
    // });

    // this.$nextTick(() => {
    // }, 300);
    // this.list
    // this.tempDeleted = Object.assign({}, this.listenerData);
    this.showModal();
  },
  beforeCreate() {
    reactive;
    this.listenerData = this.$store.getters.getModalData;
  },
};
</script>

<style lang="scss" scoped>
@font-face {
  font-family: "Comfortaa";
  src: url("../../assets/fonts/Comfortaa-Regular.ttf");
}
#ListenerModal {
  transition: 0.3s;
  width: 100vw;
  height: 100vh;
  position: absolute;
  top: 0px;
  font-family: "Comfortaa";
  overflow: hidden;

  &.hide {
    visibility: hidden;
  }

  .content {
    @include fullDimension();
    @include alignCenter();
    position: relative;
    overflow: hidden;

    .blur-bg {
      @include fullDimension();
      // background: rgba(0, 0, 0, 0.5);
      backdrop-filter: blur(0px);
      filter: blur(0px);
      position: absolute;
      z-index: 0;
    }

    .dialog {
      padding-top: 20px;
      box-sizing: border-box;
      height: 460px;
      // width: 420px;
      width: 0px;
      border-radius: 15px;
      background: $darkbg;
      overflow: hidden;
      z-index: 1;

      h2 {
        color: getTextColor(light);
        margin-left: 20px;
        font-weight: getFontWeight(regular);
      }

      .content {
        width: 100%;
        height: calc(100% - 87px);
        // background: purple;
        position: relative;

        .screen {
          // background: red;
          height: calc(100% - 50px - 15px);
          position: absolute;
          overflow-x: auto;
          overflow: hidden;
          top: 0px;
          width: 100%;
        }

        .carousel-controller {
          @include flexAlignCenter(row);
          justify-content: center;
          height: 15px;
          width: 100%;
          position: absolute;
          // background: black;
          bottom: 50px;

          .control-btn {
            border-radius: 50%;
            height: 12px;
            width: 12px;
            margin-left: 13px;
            margin-bottom: 4px;
            background: getTextColor(dark);

            &:hover {
              cursor: pointer;
            }

            &.active {
              background: $primary;
            }
          }
        }

        .footer {
          @include flexAlignCenter(row);

          height: 50px;
          box-sizing: border-box;
          width: 100%;
          position: absolute;
          bottom: 0px;
          background: #262534;

          img {
            width: 35px;
            height: 30px;
            position: absolute;
            left: 15px;
          }

          #Switch {
            position: absolute;
            left: 15px;
          }

          button {
            @include alignCenter();
            border-radius: 3px;
            padding: 9px 20px 9px 20px;
            outline: none;
            color: getTextColor(light);
            border: transparent;
            font-size: 16px;
            font-family: "Comfortaa", cursive;
            position: absolute;

            &:nth-child(2) {
              background: $darkprimary;
              right: 115px;
            }

            &:nth-child(3) {
              background: $primary;
              right: 15px;
            }
          }
        }
      }
    }
  }
}
</style>