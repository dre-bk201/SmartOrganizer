<template>
  <div id="Trash" @click.self="closeTrash">
    <div class="trash-blur">
      <img
        src="../../assets/images/trash.svg"
        @click.stop="remove"
        width="70"
        height="70"
      />
    </div>
    <!-- {{ getTrash }} -->
  </div>
</template>

<script>
import { mapGetters, mapMutations } from "vuex";
// import { invoke } from "@tauri-apps/api";
export default {
  computed: {
    ...mapGetters(["getTrash", "getListeners"]),
  },
  methods: {
    ...mapMutations(["updateTrash", "removeListener"]),
    closeTrash() {
      this.$bus.emit("remove-blur-listener");
      setTimeout(() => {
        this.updateTrash({});
      }, 500);
    },

    realign() {
      this.$store.getters.getListeners.forEach((_, index, arr) => {
        arr[index].index = index;
      });
    },

    remove() {
      const listener = JSON.parse(JSON.stringify(this.getTrash));
      this.removeListener(listener);
      // invoke("remove_listener", { index: this.index });

      // this.getListeners.forEach((listener) => {
      //   invoke("update_listener", { listener });
      // });
    },
  },
  mounted() {
    console.log("Trash mounted");
    this.$bus.emit("blur-listener", this.getTrash.index);
  },
  unmounted() {
    this.$bus.emit("remove-blur-listener");
  },
};
</script>

<style lang="scss" scoped>
#Trash {
  //   @include flexAlignCenter(column);
  //   justify-content: flex-end;
  @include fullDimension();
  //   width: 100%;
  //   height: 100%;
  position: relative;
  //   background: red;
  //   z-index: 1;

  .trash-blur {
    @include alignCenter();
    width: 100%;
    height: 50%;
    position: absolute;
    backdrop-filter: blur(15px);
    // filter: blur(5px);
    top: calc(73%);
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.513);
    img {
      margin-bottom: 60px;
    }
  }
}
</style>