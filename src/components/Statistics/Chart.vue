<template>
  <div class="Chart">
    <!-- <vue3-chart-js
      :id="doughnutChart.id"
      :type="doughnutChart.type"
      :data="doughnutChart.data"
    ></vue3-chart-js> -->
    <vue3-chart-js v-bind="{ ...chart }" />
  </div>
</template>

<script>
import Vue3ChartJs from "@j-t-mcc/vue3-chartjs";
import { useStore } from "vuex";

import { generateChart } from "../../utils";

export default {
  name: "App",
  components: {
    Vue3ChartJs,
  },
  props: {
    chartType: {
      type: String,
      required: true,
    },
  },
  setup(props) {
    const chart = generateChart(
      props.chartType,
      useStore().getters.getListeners
    );

    return {
      chart,
    };
  },
};
</script>

<style lang="scss" scoped>
.Chart {
  @include flexAlignCenter(row);
  width: calc(100% - 55px);
  min-height: 350px;
  padding: 10px;
  border-radius: 30px;
  background: $darkprimary;
  margin-left: 15px;
  margin-right: 15px;
  margin-bottom: 30px;

  // #doughnut {
  //   transform: scale(0.8);
  // }
}
</style>