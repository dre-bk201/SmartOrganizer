<template>
  <div id="ActionScreen">
    <Location
      v-for="key in Object.keys(listener.action_paths)"
      :key="key"
      :dir="key"
      :advanced="true"
      :index="listener.index"
      :listener="listener"
      :keyName="key"
    />
  </div>
</template>

<script>
import anime from "animejs";
import Location from "./Location.vue";
import { mapGetters } from "vuex";
export default {
  components: {
    Location,
  },
  props: ["name", "listener"],
  computed: { ...mapGetters(["getListeners"]) },

  mounted() {
    anime({
      targets: this.$el,
      translateX: ["400px", "0px"],
      duration: 200,
      easing: "easeInSine",
    });
  },
};
</script>

<style lang="scss" scoped>
#ActionScreen {
  padding: 35px 15px 0px 15px;
  height: 100%;
  box-sizing: border-box;
  overflow-y: auto;
}
</style>