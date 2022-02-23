<script lang="ts" setup>
import { computed, ref } from "vue";
import { useStore } from "vuex";
import anime from "animejs";

const store = useStore();

// Elementrefs
const rootElement = ref();
const broomElement = ref();

// Computed
const isCleaning = computed(() => store.getters["triggerClean"]);

// Methods
const randomRange = (min: number, max: number) =>
  Math.random() * (max - min) + min;

const onEnter = () => {
  const size = 30;
  const particles: Array<HTMLDivElement> =
    rootElement.value.querySelectorAll(".particle");

  anime({
    targets: broomElement.value,
    rotate: ["-85deg", "0deg", "-85deg", "0deg", "-85deg", "0deg"],
    easing: "linear",

    duration: 2500,
  });

  particles.forEach((particle) => {
    particle.style.bottom = `${randomRange(0, size - 10)}px`;
    particle.style.left = `${randomRange(0, size)}px`;

    anime({
      targets: ".particle",
      bottom: size + 10,
      easing: "linear",
      duration: 3000,
    });
  });
};
</script>

<template>
  <div
    ref="rootElement"
    class="broom h-10 w-10 center relative overflow-hidden"
  >
    <img ref="broomElement" src="../assets/icons/broom.svg" class="absolute" />

    <transition @enter="onEnter">
      <div v-if="isCleaning" class="h-full relative w-full overflow-hidden">
        <div class="particle" />
        <div class="particle" />
        <div class="particle" />
        <div class="particle" />
        <div class="particle" />
        <div class="particle" />
        <div class="particle" />
      </div>
    </transition>
  </div>
</template>

<style>
.particle {
  @apply bg-[#623416c7] dark:bg-[#c47c4dc7] w-1 h-1 bottom-0 rounded-full absolute;
}
</style>
