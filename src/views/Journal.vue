<script lang="ts" setup>
import Log from "../components/Log.vue";

import { computed } from "vue";
import { useStore } from "vuex";
import { Listener } from "../store/modules/listener";

const store = useStore();

const search = computed(() => store.getters["logSearch"]);

const getFilteredListeners = computed(() => {
  let arr = store.state.listener.listeners as Array<Listener>;

  return arr.filter((list) =>
    list.title.toLowerCase().includes(search.value.toLowerCase())
  );
});
</script>
<template>
  <div
    class="bg-l_primary dark:bg-d_primary rounded-tl-2xl dark:text-gray-300 h-full px-7 py-5 box-content overflow-y-auto"
  >
    <template v-for="listener in getFilteredListeners">
      <Log
        v-for="log in listener.logs"
        :key="log.id"
        :listener="listener"
        :log="log"
      />
    </template>
  </div>
</template>
