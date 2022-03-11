<script lang="ts" setup>
import Log from "../components/Log.vue";

import { computed, onUnmounted, Ref, ref, watch } from "vue";
import { useStore } from "vuex";
import { Listener } from "../store/modules/listener";
import { useDimensions, useFetchList } from "../utils";
import { useScroll } from "@vueuse/core";

const store = useStore();
const dimensions = useDimensions();

const rootElementRef = ref<HTMLDivElement | null>(null);

const { arrivedState } = useScroll(rootElementRef, {
  offset: { top: 250, bottom: 35 },
});

const low = ref(0);
const high = ref(Math.round(dimensions.height / 125));

const search = computed(() => store.getters["logSearch"]);

const filterBySearch = computed(() =>
  (store.getters["listener/listeners"] as Array<Listener>).filter((listener) =>
    listener.title.toLowerCase().includes(search.value.toLowerCase())
  )
);

const clampRise = (high: Ref<number>, len: number) => {
  const chunk = 30;
  const rise = high.value + chunk;

  if (rise > len && high.value < len) high.value += len - high.value;
  else if (rise < len) high.value = rise;
};

const fetchList = computed(() => {
  return useFetchList(filterBySearch.value, low.value, high.value);
});

watch(arrivedState, () => {
  if (arrivedState.bottom) clampRise(high, fetchList.value.len);
});
</script>
<template>
  <div
    ref="rootElementRef"
    class="bg-l_primary dark:bg-d_primary rounded-tl-2xl dark:text-gray-300 h-full px-7 py-5 box-content overflow-y-auto"
  >
    <Log
      v-for="(log, idx) in fetchList.slice"
      :key="`${idx}${log.timestamp}`"
      v-bind="{ ...log, idx }"
    />
  </div>
</template>
