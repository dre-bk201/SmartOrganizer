<script lang="ts" setup>
import anime from "animejs";
import { onMounted } from "vue";

interface Props {
  searchType: string;
  condition: string;
  text: string;
}

defineProps<Props>();

onMounted(() => {
  anime({
    targets: ".grammar",
    // fontSize: ["0px", "48px"],
    opacity: [0, 1],
    duration: 200,
    easing: "linear",
  });
});
</script>

<template>
  <div class="flex items-center w-full">
    <span class="grammar mr-7 text-black dark:text-gray-400 text-4xl font-adine"
      >If</span
    >
    <span class="grammar mr-4 text-[#5ccbff] text-5xl font-adine">{{
      searchType
    }}</span>
    <span class="grammar mr-4 text-[#ff3f85] text-5xl font-adine">{{
      condition
    }}</span>
    <span class="grammar text-[#5cfc59] text-5xl font-adine">{{ text }}</span>
    <span class="grammar ml-8 text-black dark:text-gray-400 text-4xl font-adine"
      >Then...</span
    >
  </div>
</template>
