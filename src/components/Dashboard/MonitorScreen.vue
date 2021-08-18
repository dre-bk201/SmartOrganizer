<template>
  <div id="MonitorScreen">
    <Location
      v-for="(dir, index) in listenerData.value.monitor_paths"
      :key="index"
      :dir="dir"
      :index="index"
    />
  </div>
</template>

<script>
import Location from "./Location.vue";
import anime from "animejs";

export default {
  name: "Monitor",
  components: {
    Location,
  },
  inject: ["listenerData"],
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
#MonitorScreen {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  overflow: hidden;
  padding: 30px 15px 0px 15px;
}
</style>