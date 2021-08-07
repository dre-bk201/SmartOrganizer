<template>
  <div class="Listener" @click.stop="handleClick" @click.right="deleteListener">
    <div class="status-bar" />
    <div class="listener-container">
      <img src="../../assets/images/folder-search.svg" />
      <div class="text-info">
        <div class="listener-title">{{ listener.title }}</div>
        <div class="listener-info">
          Has been reorganizer {{ listener.logs.length }} times
        </div>
      </div>

      <div class="status-info">
        <div class="as">
          <img
            :src="
              status
                ? require('../../assets/images/checkmark.svg')
                : require('../../assets/images/warning.svg')
            "
          />
        </div>
        <span>Updated</span>
        <span class="date">{{ dateUpdated }}</span>
      </div>
    </div>

    <div class="listener-count">{{ listener.monitor_paths.length }}</div>
  </div>
</template>

<script>
import { mapMutations } from "vuex";
import { invoke } from "@tauri-apps/api";
export default {
  props: ["listener", "index"],
  data: () => ({
    clicks: 0,
    timer: null,
    delay: 400,
  }),
  watch: {
    listener: {
      handler(value) {
        invoke("update_listener", {
          listener: value,
        });
      },
      deep: true,
    },
  },
  methods: {
    ...mapMutations(["updateSidepaneData", "updateModalData", "updateTrash"]),
    handleClick() {
      this.clicks++;
      if (this.clicks === 1) {
        this.timer = setTimeout(() => {
          this.showSidePane();
          // this.deleteListener();
          this.clicks = 0;
        }, this.delay);
      } else {
        clearTimeout(this.timer);
        this.showListener();
        this.clicks = 0;
      }
    },
    showSidePane() {
      let path = "";
      if (!Object.keys(this.listener.action_paths).length) {
        path = this.listener.monitor_paths[0];
      } else {
        path = Object.keys(this.listener.action_paths)[0];
      }
      const data = { loading: true, title: this.listener.title };
      this.updateSidepaneData(data);
      invoke("walk_dir", {
        path: path,
      }).then((res) => {
        res.title = this.listener.title;
        res.loading = false;
        console.log("structure: ", res);
        this.updateSidepaneData(res);
      });
    },

    showListener() {
      // this.updateModalData(this.listener);
      this.updateModalData(JSON.parse(JSON.stringify(this.listener)));
    },

    deleteListener() {
      this.updateTrash(JSON.parse(JSON.stringify(this.listener)));
    },
  },
  computed: {
    isToday() {
      const date = new Date(
        this.listener.logs[this.listener.logs.length - 1].timestamp
      );
      const year = date.getFullYear();
      const month = date.getMonth();
      const day = date.getDate();

      const dateNow = new Date();

      if (
        dateNow.getMonth() + 1 == month + 1 &&
        dateNow.getFullYear() == year &&
        dateNow.getDate() == day
      )
        return true;

      return false;
    },
    dateUpdated() {
      let prefix = "";

      if (!this.listener.logs[this.listener.logs.length - 1]?.timestamp)
        return new Date().toLocaleString("en-US", {
          year: "numeric",
          month: "numeric",
          day: "numeric",
        });

      let date = new Date(
        this.listener.logs[this.listener.logs.length - 1]?.timestamp
      );
      let today = new Date();

      let time = date.toLocaleString("en-US", {
        hour: "numeric",
        minute: "numeric",
        hour12: true,
      });
      let yesterday = new Date(today - 864e5);

      if (this.isToday) prefix = "Today";
      else if (
        yesterday.getDate() == date.getDate() &&
        yesterday.getMonth() == date.getMonth() &&
        yesterday.getFullYear() == date.getFullYear()
      )
        prefix = "Yesterday";
      else {
        prefix = "";
        time = date.toLocaleString("en-US", {
          year: "numeric",
          month: "numeric",
          day: "numeric",
        });
      }

      return `${prefix} ${time}`;
    },

    status() {
      if (!this.listener.logs[this.listener.logs.length - 1]) return false;
      const date = new Date(
        this.listener.logs[this.listener.logs.length - 1].timestamp
      );
      const year = date.getFullYear();
      const month = date.getMonth();
      const day = date.getDate();

      const dateNow = new Date();

      if (
        dateNow.getMonth() + 1 >= month + 1 &&
        dateNow.getFullYear() >= year &&
        dateNow.getDate() - day >= 3
      )
        return false;

      return true;
    },
  },
  mounted() {
    this.$bus.on("blur-listener", (index) => {
      if (this.listener.index != index) {
        console.log(`${this.listener.title} is blurring`);
        this.$el.classList.add("blur-listener");
        console.log(this.$el.classList);
      }
    });

    this.$bus.on("remove-blur-listener", () => {
      this.$el.classList.remove("blur-listener");
    });

    invoke("add_listener", {
      listener: this.listener,
    });
  },
};
</script>

<style lang="scss" scoped>
.Listener {
  height: 80px;
  width: 100%;
  background: $darkprimary;
  border-radius: 5px;
  user-select: none;
  margin-bottom: 35px;
  box-shadow: $boxshadow;
  transition: 0.5s;
  position: relative;
  font-family: Comfortaa;

  &.blur-listener {
    filter: blur(8px);
  }

  .status-bar {
    height: 3px;
    width: 100%;
    background: $success;
    border-radius: 4px 4px 0px 0px;
  }

  img:nth-child(1) {
    margin-right: 10px;
  }

  .listener-container {
    @include fullDimension();
    @include flexAlignCenter(row);
    padding: 0px 35px 0px 15px;
    box-sizing: border-box;

    .text-info {
      @include flexAlign(column, center, flex-start);
      height: 100%;
      flex-grow: 1;
      padding-left: 10px;

      .listener-title {
        color: getTextColor(dim);
        font-size: 23px;
        margin-bottom: 5px;
      }

      .listener-info {
        color: $primary;
        font-size: 11px;
      }
    }

    .status-info {
      @include flexAlign(column, center, center);
      flex-direction: column;
      margin-left: 10px;
      box-sizing: border-box;
      font-size: 11px;
      color: white;
      font-family: Comfortaa;

      .date {
        font-family: Comfortaa;
        // font-size: 11px;
      }

      img {
        width: 40px;
        margin-bottom: 2px;
        margin-right: 0px;
      }
    }
  }

  .listener-count {
    @include alignCenter();
    background: $primary;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    position: absolute;
    bottom: -10px;
    box-sizing: border-box;
    font-size: 11px;
    right: 0px;
    color: white;
  }
}
</style>