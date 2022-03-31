<script lang="ts" setup>
import DetailCard from "../DetailCard.vue";
import RulesPopup from "../RulesPopup.vue";
import Grammar from "../Grammar.vue";

import { useStore } from "vuex";
import { computed, ref, onMounted, ComponentPublicInstance, inject } from "vue";
import { Rule } from "../../store/modules/listener";

import edit_rule from "../../assets/icons/edit_rule.svg";
import edit_rule_dark from "../../assets/icons/edit_rule_dark.svg";
import anime from "animejs";

const store = useStore();

// Variables
let selectionRef = ref<ComponentPublicInstance<HTMLDivElement> | null>();

// Computed
const getRules = computed(() => store.state.modal.listener.rules);

const getCurrentRule = computed(() => store.state.modal.currentRule);

const getSelection = computed(() => store.state.modal.listener.selection);

const isDark = computed(() => store.getters["config/isDark"]);

// Functions
const removeRule = (idx: number) => store.dispatch("modal/removeRule", idx);

const editRule = (idx: number, rule: Rule) =>
  store.dispatch("modal/setCurrentRule", { ...rule, idx });

const handleInputChange = (e: Event) => {
  let input = e.target as HTMLInputElement;
  store.dispatch("modal/setSelection", input.id);
};

const onLeave = (el: Element, done: () => void) =>
  anime({
    targets: el,
    opacity: 0,
    duration: 300,
    easing: "linear",
  }).finished.then(() => {
    done();
  });

const onEnter = (el: Element, done: () => void) => {
  const leftBlock = el.querySelector(".left--block");
  const rightBlock = el.querySelector(".right--block");

  anime({
    targets: leftBlock,
    translateY: ["-100%", "0px"],
    duration: 500,
  }).finished.then(() => {
    anime({
      targets: [leftBlock, rightBlock],
      opacity: [1, 0],
      duration: 100,
      easing: "linear",
    }).finished.then(() => {
      (<HTMLDivElement>leftBlock).style.transform = "translateY(-100%)";
      (<HTMLDivElement>rightBlock).style.transform = "translateY(100%)";
      done();
    });
  });

  anime({
    targets: rightBlock,
    translateY: ["100%", "0px"],
    duration: 500,
  });
};

onMounted(() => {
  let root: HTMLInputElement = selectionRef.value?.$el;

  root.querySelectorAll("input[id]").forEach((ele) => {
    if (getSelection.value == ele.id) (ele as HTMLInputElement).checked = true;
    else (ele as HTMLInputElement).checked = false;
  });
});
</script>
// TODO: Add icon for text to filter for on Modal
<template>
  <div class="h-screen w-full flex flex-col">
    <h1 class="title text-xl mb-7 pl-8">Rules</h1>
    <!-- <Teleport to="#modal"> -->
    <Transition @enter="onEnter" @leave="onLeave">
      <RulesPopup
        v-if="Object.keys(getCurrentRule).length"
        v-bind="{ ...getCurrentRule }"
      />
    </Transition>
    <!-- </Teleport> -->
    <DetailCard
      ref="selectionRef"
      class="ml-8 mr-8 text-gray-500 dark:text-gray-300"
    >
      <template #header>
        <div class="relative flex items-center p-[3px]">
          <h1 class="pl-3 text-sm">Selection</h1>
        </div>
      </template>

      <template #content>
        <div ref="" class="px-2 py-3 flex items-center">
          <input
            id="Any of the following"
            @input="handleInputChange($event)"
            class="ml-3"
            type="radio"
            name="selection"
          />
          <label class="ml-1" for="Any of the following"
            >Any of the following
          </label>

          <input
            id="All of the following"
            @change="handleInputChange"
            class="ml-3"
            type="radio"
            name="selection"
          />
          <label class="ml-1" for="All of the following"
            >All of the following
          </label>
        </div>
      </template>
    </DetailCard>

    <div class="content items-center overflow-y-auto">
      <DetailCard
        class="ml-8 mr-8 text-gray-500 dark:text-gray-300"
        v-for="(rule, idx) in getRules"
      >
        <template #header>
          <div class="header relative flex items-center p-[3px]">
            <span class="header pl-3 text-sm"
              >Grammar of Rule #{{ idx + 1 }}</span
            >
            <button
              @click="removeRule(idx)"
              class="absolute right-6 text-xs hover:text-[#FF0303]"
            >
              Remove
            </button>
          </div>
        </template>

        <template #content>
          <div class="px-2 relative py-2 flex items-center">
            <!-- <span v-for="item in parseRule(rule)" class="font-bold">
              {{ item }}
            </span> -->
            <Grammar class="font-bold" v-bind="{ ...rule }" />

            <img
              @click="editRule(idx, rule)"
              class="absolute right-[-20px] hover:cursor-pointer"
              :src="isDark ? edit_rule_dark : edit_rule"
            />
          </div>
        </template>
      </DetailCard>
    </div>
  </div>
</template>
