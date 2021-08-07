<template>
  <div id="Switch" @click="enableSmartOrganizer">
    <div class="fill" />
    <div class="knob">
      <div class="indicator" />
    </div>
  </div>
</template>

<script>
import anime from "animejs";
export default {
  props: {
    enable: {
      type: Boolean,
      // required: true,
    },
    setEnable: {
      type: Function,
      // required: true,
    },
  },
  methods: {
    enableSmartOrganizer() {
      const root = this.$el;
      root.classList.toggle("toggle");
      if (root.classList.contains("toggle")) this.onAnimation();
      else this.offAnimation();

      // this.setEnable(root.classList.contains("toggle"));
    },
    onAnimation() {
      const knob = this.$el.querySelector(".knob");
      anime({
        targets: knob,
        right: "0",
        easing: "easeInSine",
        duration: 300,
      });

      anime({
        targets: ".fill",
        width: "100%",
        duration: 300,
        easing: "easeInSine",
      });

      anime({
        targets: knob.querySelector(".indicator"),
        backgroundColor: "#5BFF58",
        boxShadow: {
          value: "0px 0px 10px #5BFF58",
          duration: 100,
        },
      });
    },
    offAnimation() {
      const knob = this.$el.querySelector(".knob");
      anime({
        targets: knob,
        right: "25px",
        easing: "easeInSine",
        duration: 300,
      });

      anime({
        targets: ".fill",
        width: "0%",
        duration: 300,
        easing: "easeInSine",
      });

      anime({
        targets: knob.querySelector(".indicator"),
        backgroundColor: "#FF3B76",
        boxShadow: {
          value: "0px 0px 10px #FF3B76",
          duration: 100,
        },
      });
    },
    reset() {},
  },
};
</script>

<style lang="scss" scoped>
#Switch {
  height: 30px;
  width: 54px;
  background: #505050;
  border-radius: 30px;
  overflow: hidden;
  position: relative;

  .knob {
    border-radius: 50%;
    background: #424242;
    height: 30px;
    width: 30px;
    position: absolute;
    display: grid;
    place-items: center;
    right: calc(100% - 30px);

    .indicator {
      width: 7px;
      height: 7px;
      border-radius: 50%;
      background: #ff3b76;
      box-shadow: 0px 0px 10px #ff3b76;
    }
  }

  .fill {
    position: absolute;
    height: 100%;
    width: 0px;
    border-radius: 30px;

    background: #6a5dff;
  }
}
</style>