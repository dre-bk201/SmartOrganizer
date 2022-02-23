<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import { Listener } from "../store/modules/listener";
import { useStore } from "vuex";
import anime from "animejs";

const props = defineProps<{ listener: Listener }>();
const store = useStore();

let animate = ref(false);

const calcPercentage = computed(() => {
  let total = store.getters["listener/getNumOfLogs"];
  let portion = props.listener.logs.length;

  if (portion && total) return Math.round((portion / total) * 100);
  return 0;
});

const onEnter = (e: Element) => {
  anime({
    targets: e,
    width: `${calcPercentage.value}%`,
    easing: "linear",
    duration: 500,
  });
};

onMounted(() => {
  setTimeout(() => {
    animate.value = true;
  }, 500);
});
</script>

<template>
  <div
    class="usage relative flex items-center rounded-xl bg-l_primary dark:bg-d_primary overflow-hidden mb-5 h-12"
  >
    <transition @enter="onEnter">
      <div v-show="animate" class="progress h-full" />
    </transition>
    <span class="pl-5 flex center absolute font-bold">
      {{ listener.title }}
      <span class="ml-4">{{ calcPercentage }}%</span>
    </span>
  </div>
</template>

<style scoped>
.usage {
  box-shadow: 2px 2px 0px 0px rgba(0, 0, 0, 0.25);
}
</style>
