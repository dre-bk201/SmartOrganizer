<script setup lang="ts">
import dark_inactive from "../../assets/img/inactive-listener-dark.png";
import dark_active from "../../assets/img/active-listener-dark.png";
import light_inactive from "../../assets/img/inactive-listener-light.png";
import light_active from "../../assets/img/active-listener-light.png";

import { computed, inject } from "vue";
import { format } from "date-fns";

import { ListenerStoreKey, ThemeKey } from "../../store";
import { useStatus } from "../../composables";
import { IListener } from "../../models";
import { STATUSCOLORS } from "../../static";
import { firstBy } from "thenby";

const props = defineProps<IListener>();
defineEmits<{
  setActiveId: [id: string];
}>();

const { isDark } = inject(ThemeKey)!;
const { listener } = inject(ListenerStoreKey)!;
const status = useStatus(props);

const organized = computed(
  () => listener.logs.filter((log) => log.parentId == props.id).length
);
const isActive = computed(() => status.value === "ACTIVE");
const actions = computed(
  () => new Set(props.actions.map(({ action }) => action))
);
const createdDate = computed(() => format(new Date(props.created), "PPp"));
const lastUpdated = computed(() => {
  let logs = listener.logs.filter((l) => l.parentId == props.id);
  let recent = logs.sort(firstBy((a, b) => a.timestamp.localeCompare(b.timestamp), "desc"))[0];
  if (recent) return format(new Date(recent.timestamp), "PPp");
  return "NONE";
});
</script>

<template>
  <div class="rounded-lg adaptive--dark pl-3 py-1 pr-6 flex items-center dark:shadow-xl shadow-sm">
    <img v-show="isDark" class="w-20 h-16 mr-6" :src="isActive ? dark_active : dark_inactive" />
    <img v-show="!isDark" class="w-20 h-16 mr-6" :src="isActive ? light_active : light_inactive" />

    <div class="flex-grow flex flex-col gap-1">
      <h1 class="text-lg font-semibold">
        {{ title }}
        <span class="text-sm ml-2 rounded-md py-[4px] px-2 adaptive--dark--invert"
          :style="{ color: STATUSCOLORS[status] }">
          {{ status }}
        </span>
      </h1>

      <p>
        <span class="rounded-md text-gray-800 bg-blue-500 p-[4px] mr-1" v-for="action in actions">
          {{ action }}
        </span>

        Organized
        <span class="adaptive--dark--invert text-sm px-2 rounded-md dark:text-blue-500 py-[1px]">{{ organized }}</span>
        item(s)
      </p>
      <p class="font-semibold">
        Last updated:
        <span class="date text-primary">{{ lastUpdated }}</span> |
        Created:
        <span class="date text-green-400">{{ createdDate }}</span>
      </p>
    </div>

    <img v-show="status == 'DRAFT'" width="50" class="mr-6" src="../../assets/img/alert-circle.svg" />
  </div>
</template>

<style scoped>
.date {
  @apply text-sm;
}
</style>
