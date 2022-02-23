<script lang="ts" setup>
import Chart from "../components/Chart.vue";
import ListenerUsage from "../components/ListenerUsage.vue";

import { useStore } from "vuex";
import { computed } from "vue";

const store = useStore();
const listeners = computed(() => store.state.listener.listeners);
</script>
<template>
  <div
    class="bg-l_primary flex dark:bg-d_primary rounded-tl-2xl dark:text-gray-300 h-full px-3 pb-2 box-content overflow-y-auto"
  >
    <div class="graphs flex flex-col grow px-4 overflow-y-auto pt-6">
      <div
        class="graph-1 min-h-[25rem] rounded-md bg-l_white dark:bg-d_secondary mb-5"
      >
        <Chart chartType="line" :listeners="listeners" />
      </div>
      <div
        class="graph-2 min-h-[25rem] rounded-md bg-l_white dark:bg-d_secondary"
      >
        <Chart chartType="doughnut" :listeners="listeners" />
      </div>
    </div>
    <div
      class="detail h-full bg-l_white dark:bg-d_secondary w-[225px] rounded-md px-4 overflow-y-auto"
    >
      <header class="text-center pt-6 pb-7 font-semibold text-2xl">
        Listener Usage
      </header>

      <ListenerUsage :listener="listener" v-for="listener in listeners">
      </ListenerUsage>
    </div>
  </div>
</template>
