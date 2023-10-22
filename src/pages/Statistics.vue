<script setup lang="ts">
//@ts-nocheck
import Chart from 'primevue/chart';
import StatsFolder from "~icons/stats_folder";
import StatsFile from "~icons/stats_file";

import { inject, onMounted, reactive, ref } from 'vue';
import { ListenerStoreKey } from '../store';
import { stringToColour } from "../utils";
import { format } from 'date-fns';

const { listener } = inject(ListenerStoreKey)!;

let chartData = reactive({
  labels: [] as string[],
  datasets: [
    {
      label: "Listeners",
      data: [] as number[],
      borderWidth: 2
    }
  ]
});

const chartOptions = ref({
  scales: {
    y: {
      beginAtZero: true,
    },
  },
});

function generateChartData(listeners: IListener[], logs: ILog[]) {

  listeners.forEach(listener => {
    chartData.labels.push(listener.title);
    chartData.datasets[0].data.push(logs.filter(log => log.parentId == listener.id).length);
  });

}

onMounted(() => {
  generateChartData(listener.listeners, listener.logs);
});

</script>

<template>
  <main class="px-5 h-0 py-8 flex-grow flex flex-col gap-5 overflow-y-auto">
    <section class="flex rounded-md relative dark:bg-dark !bg-opacity-70 p-6 gap-5 lg:h-[400px] md:h-[250px]">
      <h1 class="text-primary font-bold absolute left-11 top-[-15px] shadow-2xl px-2 py-1 rounded-md bg-[#303136]">
        Files affected for listener per day
      </h1>

      <Chart class="flex-grow rounded-md p-1" type="bar" :data="chartData" :options="chartOptions" />

      <div class="dark:bg-darker !bg-opacity-50 flex-shrink-0 min-w-[250px] max-w-3xl rounded-md p-2 flex flex-col gap-2">
        <h1>Keys for listeners </h1>
        <hr />
        <ul class="flex flex-col overflow-y-auto gap-4">
          <li v-for="l, idx in listener.listeners" class="items-center gap-2">
            <span class="w-2 h-2 rounded-md inline-block" :style="{ background: stringToColour(l.title + l.id) }" />
            {{ l.title }} #{{ idx + 1 }}
          </li>
        </ul>
      </div>
    </section>

    <section class="flex gap-5">
      <section class="flex flex-col flex-grow gap-5">
        <div class="f1 text-white relative flex flex-col pl-5 py-5 gap-3 rounded-lg bg-indigo-400">
          <div class="overlay absolute top-0 left-0 h-full w-full"></div>
          <StatsFolder class="text-lg" />
          <span class="text-lg">Folders Affected</span>
          <span class="font-bold">0</span>
        </div>
        <div class="f2 text-white  relative flex flex-col pl-5 py-5 gap-3 rounded-lg bg-indigo-400">
          <div class="overlay absolute top-0 left-0 h-full w-full"></div>
          <StatsFile class="text-lg" />
          <span class="text-lg">Files Organized</span>
          <span class="font-bold">{{ listener.logs.length }}</span>
        </div>
      </section>
      <div class="rounded-md flex flex-col gap-3 flex-grow adaptive--dark px-4 py-4">
        <h1 class="text-xl dark:text-white font-bold">Listener Highlights</h1>

        <span class="gridlayout text-gray-400">
          <span>Folder/File</span>
          <span>Actions</span>
          <span>Time</span>
        </span>

        <section class="flex-grow flex divide-y divide-gray-600 text-gray-400 flex-col h-0 overflow-y-auto">
          <div class="gridlayout relative py-4" v-for="log in listener.logs">
            <span class="text-ellipsis rtl-grid overflow-hidden whitespace-nowrap mr-4">{{ log.path.split("/").at(-1)
            }}</span>
            <span>{{ log.action }}</span>
            <span>{{ format(new Date(log.timestamp), "PPpp") }}</span>
          </div>
        </section>
      </div>
    </section>
  </main>
</template>

<style scoped>
.gridlayout {
  display: grid;
  grid-template-columns: 0.55fr 100px 0.4fr;
}

.rtl-grid {
  direction: rtl;
}

.f1 {
  background: linear-gradient(85.9deg, #8337FF 1.1%, #9656FF 51.75%, #9C76DA 102.37%);
}

.f2 {
  background: linear-gradient(86.09deg, #3F4FDB 4.16%, #5A66D0 42.86%, #727DDD 96.62%);
}

.overlay {
  background: url("../../src/assets/img/overlay.png");
}
</style>
