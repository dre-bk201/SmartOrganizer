<template>
  <div class="ListenerPerc">
    <div class="percentage" ref="percentage"></div>
    <div class="text">
      <p>
        {{ listener.title }}
      </p>
      <span class="perc">&nbsp; {{ calcPercentage }}%</span>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import anime from "animejs";
export default {
  props: ["listener"],
  computed: {
    ...mapGetters(["getTotalLogsCount"]),
    calcPercentage() {
      let total = this.getTotalLogsCount;
      let portion = this.listener.logs.length;
      return Math.round((portion / total) * 100);
    },
  },
  mounted() {
    anime({
      targets: this.$refs.percentage,
      width: ["0%", `${this.calcPercentage}%`],
      duration: 600,
      easing: "easeInOutQuad",
    });
  },
};
</script>

<style lang="scss" scoped>
.ListenerPerc {
  @include flexAlignCenter(row);
  height: 45px;
  background: $darkbg;
  border-radius: 8px;
  margin-bottom: 30px;
  position: relative;

  .percentage {
    border-radius: 8px;
    height: 100%;
    background: linear-gradient(
      90deg,
      #5096ff 7.05%,
      rgba(80, 150, 255, 0.4) 106.41%
    );
    // width: 30%;
    position: absolute;
  }

  .text {
    position: absolute;
    p {
      padding-left: 10px;
      font-family: Comfortaa;
      display: inline-block;
    }
  }
}
</style>