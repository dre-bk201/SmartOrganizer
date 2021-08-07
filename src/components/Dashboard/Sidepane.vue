<template>
  <!-- Add a loader for sidepane where it shows the sidepane by passing blank content and it will show a loader  -->

  <div
    id="Sidepane"
    :class="Object.keys(getSidepaneData).length ? 'visible' : ''"
  >
    <div class="sidepane-content">
      <div v-show="getSidepaneData.loading" class="loading-screen">Loading</div>
      <div v-show="!getSidepaneData.loading" class="title">
        {{ getSidepaneData.title }}
      </div>
      <div v-show="!getSidepaneData.loading">
        <!-- {{ getSidepaneData }} -->
        <TreeBrowser :node="getSidepaneData" />
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import TreeBrowser from "./Treebrowser.vue";
export default {
  components: {
    TreeBrowser,
  },
  computed: {
    ...mapGetters(["getSidepaneData"]),
  },
  data: () => ({
    length: 0,
  }),
  mounted() {
    console.log(this.getSidepaneData);
    //   console.log(this.$el.target);
  },
  // beforeDestroy() {
  //   this.$el.classList.remove("visible");
  // },
};
</script>

<style lang="scss" scoped>
#Sidepane {
  transition: 0.3s;
  height: 100vh;
  width: 0px;
  background: $darkprimary;
  max-width: 300px;

  &.visible {
    padding-right: $padding;
    width: calc(50%);
  }

  .sidepane-content {
    background: $darkbg;
    margin-bottom: $padding;
    height: calc(100% - #{$padding});
    width: 100%;
    border-radius: 8px;
    color: white;
    padding: 10px;
    box-sizing: border-box;

    overflow-y: auto;

    .title {
    }
  }
}
</style>