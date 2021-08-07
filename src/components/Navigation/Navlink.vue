<template>
  <router-link :to="route" @click="animate">
    <div v-html="icon"></div>
    <!-- <div v-html="icon" @click.stop="animate"></div> -->
    <!-- <span @click="animate"> {{ name }}</span> -->
    <span> {{ name }}</span>
  </router-link>
</template>

<script>
import anime from "animejs";
export default {
  props: ["route", "name", "distance"],
  data: () => ({
    bar: null,
    icon: null,
  }),
  methods: {
    animate() {
      anime({
        targets: ".container .bar",
        translateY: this.distance,
      });
    },
  },
  mounted() {
    if (this.name.toLowerCase() == "dashboard") {
      const { dashboard } = require("../../icons");
      this.icon = dashboard;
    } else if (this.name.toLowerCase() == "logs") {
      const { logs } = require("../../icons");
      this.icon = logs;
    } else if (this.name.toLowerCase() == "statistics") {
      const { statistics } = require("../../icons");
      this.icon = statistics;
    }
  },
};
</script>


<style lang="scss">
a {
  @include flexAlignCenter(column);

  width: 100%;
  font-size: 12px;
  fill: getTextColor(dark);
  color: getTextColor(dark);
  margin-bottom: 40px;
  height: 65px;
}

a.router-link-exact-active {
  fill: $primary;
  color: $primary;
}
</style>