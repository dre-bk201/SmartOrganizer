<script lang="ts" setup>
import { computed } from "vue";
import { useStore } from "vuex";

import day from "../assets/icons/day.svg";
import night from "../assets/icons/night.svg";

const store = useStore();

// Computed
const isDark = computed(() => store.getters["config/isDark"]);

// Function
const toggleTheme = () =>
  document.getElementById("app")?.classList.toggle("dark");
</script>

<template>
  <div
    @click="toggleTheme"
    class="hover:cursor-pointer theme--switch w-12 h-7 flex items-center"
  >
    <div class="track flex items-center bg-gray-600">
      <div
        :class="`theme--knob bg-l_white dark:bg-d_primary p-[2px] ${
          isDark ? 'translate-x-5' : ''
        }`"
      >
        <img :src="isDark ? night : day" alt="" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.track {
  width: 100%;
  height: 18px;
  border-radius: 30px;
}
.theme--knob {
  transition: 0.15s ease-in;
  width: 30px;
  height: 30px;
  border-radius: 50%;
}
</style>
