<script lang="ts" setup>
import { computed, ref } from "vue";
import { useStore } from "vuex";
import Icon from "./Icon.vue";

interface Props {
  icon: string;
  title: string;
  caption: string;
  expandable: boolean;
  children?: string;
  onclick?: Function;
  expansion?: number;
}

const props = defineProps<Props>();

// Effects
const store = useStore();

// Variables
const isExpanded = ref(false);

// Computed
const isDark = computed(() => store.getters["config/isDark"]);

// Funcitons
const toggleExpansion = () => {
  if (props.expandable) isExpanded.value = !isExpanded.value;
};
</script>

<template>
  <div
    @click="() => onclick?.()"
    :class="`Tile bg-l_white dark:text-gray-200 dark:bg-d_secondary px-2 pt-4 ${
      isExpanded ? '' : 'pb-4 '
    } font-comfortaa relative mb-2 flex flex-col`"
  >
    <div class="flex">
      <!-- Icon -->
      <img class="ml-2 mr-5" :src="icon" width="35" />
      <div class="flex flex-col h-full justify-center">
        <header>
          {{ title }}
        </header>

        <span class="text-sm text-gray-600 mt-2 dark:text-gray-400">
          {{ caption }}
        </span>

        <Icon
          @click="toggleExpansion"
          name="chevron"
          :class="`absolute right-5 hover:cursor-pointer ${
            isExpanded ? 'rotate-90' : ''
          }`"
          :key="Number(isDark)"
          width="30"
          height="30"
          :fill="isDark ? 'white' : 'black'"
        />
      </div>
    </div>
    <Transition>
      <div
        v-if="isExpanded"
        :class="`dark:text-gray-300 ${
          isExpanded ? (expansion ? `fit-content` : 'h-10') : 'h-10'
        }`"
      >
        <slot></slot>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.Tile {
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
  border-radius: 3px;
}
</style>
