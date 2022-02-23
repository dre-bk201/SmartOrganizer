<script lang="ts">
import { ComputedRef, defineComponent } from "vue";

export default defineComponent({
  name: "Dashboard",
});
</script>

<script lang="ts" setup>
import ListenerVue from "../components/Listener.vue";
import ListenerDetail from "../components/ListenerDetail.vue";

import { computed } from "vue";
import { useStore } from "vuex";
import anime from "animejs";

import type { Listener } from "../store/modules/listener";

const store = useStore();

const search = computed(() => store.getters["dashboardSearch"]);

const modalListener = computed(() => store.state.modal["listener"]);

const listenerRect: ComputedRef<DOMRect> = computed(
  () => store.getters["listenerRect"]
);

const getFilteredListeners = computed(() => {
  let arr = store.state.listener.listeners as Array<Listener>;

  return arr.filter((list) =>
    list.title.toLowerCase().includes(search.value.toLowerCase())
  );
});

const onEnter = async (el: Element) => {
  const { bottom, left, width, height } = listenerRect.value;
  console.log("Inner Height", window.innerHeight);

  (<HTMLInputElement>el).style.left = `${left}px`;
  (<HTMLInputElement>el).style.bottom = `${window.innerHeight - bottom}px`;
  (<HTMLInputElement>el).style.width = `${width}px`;
  (<HTMLInputElement>el).style.height = `${height}px`;

  await anime({
    targets: el,
    duration: 3000,
    left: "0px",
    bottom: "0px",
    width: "100%",
    height: "100%",
    easing: "linear",
  }).finished;
};

const onLeave = async (el: Element) => {
  const { bottom, left, width, height } = listenerRect.value;

  (<HTMLDivElement>el).style.left = `${left}px`;
  (<HTMLDivElement>el).style.bottom = `${window.innerHeight - bottom}px`;
  (<HTMLDivElement>el).style.width = `${width}px`;
  (<HTMLDivElement>el).style.height = `${height}px`;

  // await anime({
  //   targets: el,
  //   duration: 2000,
  //   left: `${left}px`,
  //   bottom: `${window.innerHeight - bottom}px`,
  //   width: `${width}%`,
  //   height: `${height}%`,
  //   easing: "linear",
  // }).finished;
};
</script>
<template>
  <div
    class="flex flex-col flex-grow overflow-hidden rounded-tl-2xl h-full px-[1.75rem] pt-8 bg-l_primary dark:bg-d_primary overflow-y-auto"
  >
    <ListenerVue
      v-for="listener in getFilteredListeners"
      :key="listener.id"
      v-bind="{ ...listener }"
    />
    <!-- <Transition @enter="onEnter" @leave="onLeave"> -->
    <ListenerDetail v-if="Object.keys(modalListener).length" />
    <!-- </Transition> -->
  </div>
</template>
