<script setup lang="ts">
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from "@/components/ui/tooltip";

import { computed } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const props = defineProps<{ to: string, name: string, isPinned: boolean }>();

const isActive = computed(() => router.currentRoute.value.name == props.name);
const tooltipText = computed(() => props.isPinned ? props.name : '');
</script>

<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger>
        <router-link :to="to" class="navitem"
          :class="[isPinned ? 'navitem--pinned' : 'navitem--unpinned', isActive && 'dark:bg-[#424350] bg-light_hl']">
          <span :class="`highlight--block ${isActive ? 'w-[9px] mr-2' : 'w-0 mr-4 '}`" />
          <slot :isActive="isActive" name="icon"></slot>

          <div v-show="!isPinned" class="text-base block" :class="[isActive && 'text-blue-500']">
            <slot>{{ name }}</slot>
          </div>
        </router-link>
      </TooltipTrigger>
      <TooltipContent :class="['bg-primary', tooltipText.length || 'p-0']" side="right" side-offset="1">{{ tooltipText }}</TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>

<style scoped>
.navitem {
  @apply w-full flex items-center relative text-2xl py-4 rounded-md dark:hover:bg-[#424349] hover:bg-light_hl dark:text-gray-200 text-gray-700;
}

.navitem--unpinned {
  @apply pl-5 gap-3;
}

.navitem--pinned {
  @apply justify-center w-[60px];
}

.highlight--block {
  @apply absolute left-0 ease-in transition-all rounded-r-lg h-8 bg-blue-500;
}
</style>
