<script lang="ts" setup>
import Icon from "./Icon.vue";

import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";

const props = defineProps<{
  name: string;
  path: string;
  icon: string;
  width?: string;
  height?: string;
  isHovering: boolean;
}>();

const route = useRoute();

const isCurrentPage = computed(() => props.name == route.name);
</script>

<template>
  <router-link
    ref="root"
    :class="`text-white dark:text-gray-300 link ${
      isCurrentPage ? 'bg-[#676767]' : ''
    }`"
    :to="path"
  >
    <div
      :class="`${
        isHovering && isCurrentPage ? 'w-[10px] ml-[1px]' : 'w-[0px] m-0'
      } pointer bg-l_primary dark:bg-[##5C5C5C]`"
    />
    <Icon class="mr-2 pl-2" :name="icon" width="35" height="35" />
    <span v-show="isHovering"> {{ name }} </span>
  </router-link>
</template>
