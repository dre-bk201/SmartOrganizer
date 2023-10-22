<script setup lang="ts">
import LogoIcon from "~icons/logo";
import { ref, inject } from "vue";
import anime from "animejs";
import { watch } from "vue";
import { Ref } from "vue";

defineProps<{ isPinned: boolean }>();

let bubbles = ref<HTMLElement | null>();
let shouldClean: Ref<boolean> = inject("triggerCleaning")!;

watch(shouldClean, (newv) => {
  if(newv) animate();
})

function animate() {
  if (!bubbles.value) return;
  const bubbleElements: NodeListOf<HTMLDivElement> = bubbles.value.querySelectorAll(".bubble");
  const size = 60;

  anime({
    targets: ".broom",
    rotate: ["-85deg", "0deg", "-85deg", "0deg", "-85deg", "0deg"],
    easing: "linear",
    duration: 2500,
  });

  bubbleElements.forEach(async bubble => {
    bubble.style.bottom = `${randomRange(-15, size / 3)}px`
    bubble.style.left = `${randomRange(-5, size / 1.5)}px`;
    await anime({
      targets: ".bubble",
      bottom: size + 15,
      easing: "linear",
      duration: 3000,
    }).finished;

    shouldClean.value = false
  })

}

function randomRange(min: number, max: number) {
  return Math.random() * (max - min) + min;
}
</script>

<template>
  <div class="logo overflow-hidden adaptive--darker" :class="[isPinned && 'logo--pinned']">
    <span class="flex gap-3 font-bold relative">
      <div class="relative">
        <div ref="bubbles" class="absolute h-full w-full">
          <div v-for="_ in 8" class="bubble z-10 absolute bottom-[-50px]" />
        </div>

        <LogoIcon class="text-3xl broom" />
      </div>
      <span v-show="!isPinned" class="text-sm text-gray-600 dark:text-gray-300">
        SMART
        <span class="text-blue-500">ORGANIZATION</span>
      </span>
    </span>
  </div>
</template>

<style scoped>
.logo {
  @apply dark:!bg-[#424349] mb-5 p-3 rounded-lg dark:shadow-lg dark:shadow-darker h-[60px] shadow-darker shadow-sm;
}

.logo--pinned {
  @apply box-border items-center flex justify-center;
}

.logo--pinned svg {
  @apply text-3xl;
}

.bubble {
  @apply bg-[#623416c7] dark:bg-[#c47c4dc7] w-2 h-2 bottom-[-20px] rounded-full absolute;
  ;
}
</style>
