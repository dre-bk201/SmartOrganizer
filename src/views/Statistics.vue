<template>
  <div class="Statistics">
    <div class="Statistics__body">
      <div class="Statistics__body__left">
        <Chart v-if="getListeners.length" chartType="line" />
        <Chart v-if="getListeners.length" chartType="doughnut" />
      </div>
      <div class="Statistics__body__right">
        <div class="Statistics__body__right__stats-pane">
          <div class="Statistics__body__right__stats-pane__approximation">
            Saving you ~{{ savedTime }}
          </div>
          <ListenerPerc
            v-for="listener in getListeners"
            :key="listener.index"
            :listener="listener"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import ListenerPerc from "../components/Statistics/ListenerPerc.vue";
import Chart from "../components/Statistics/Chart.vue";
import { mapGetters } from "vuex";
export default {
  name: "Statistics",
  components: { ListenerPerc, Chart },
  computed: {
    ...mapGetters(["getListeners"]),
    savedTime() {
      return "30 mins";
    },
  },
};
</script>

<style lang="scss" scoped>
.Statistics {
  border-radius: 30px 0px 0px;
  background: $darkbg;
  height: 100%;
  width: calc(100vw - #{$nav-width} - #{$padding});
  position: absolute;
  top: 0px;
  left: $nav-width;
  box-sizing: border-box;
  color: white;

  &__body {
    height: 100%;
    display: flex;
    padding: 20px 10px 0px 0px;
    box-sizing: border-box;

    &__left {
      width: calc(100% - 245px);
      height: 100%;
      overflow-y: auto;
    }

    &__right {
      height: calc(100% - 15px);
      width: 250px;
      margin-left: 5px;
      &__stats-pane {
        @include fullDimension();
        box-sizing: border-box;
        padding: 150px 10px 0px 10px;
        border-radius: 8px;
        background: $darkprimary;

        &__approximation {
          font-weight: getFontWeight(bold);
          text-align: center;
          color: $success;
          margin-bottom: 30px;
        }
      }
    }
  }
}
</style>