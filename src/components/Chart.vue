<script lang="ts" setup>
import Vue3ChartJs from "@j-t-mcc/vue3-chartjs";
import { Listener } from "../store/modules/listener";
import { generateChart } from "../chart-utils";
import { ref } from "vue";

const props = defineProps<{ chartType: string; listeners: Listener[] }>();

let chart = ref(generateChart(props.chartType, props.listeners));
</script>

<template>
  <div class="Chart h-full">
    <vue3-chart-js v-bind="{ ...chart }" />
  </div>
</template>
