<script lang="ts" setup>
import Path from "../Path.vue";
import Icon from "../Icon.vue";
import DetailCard from "../DetailCard.vue";

import { shell } from "@tauri-apps/api";
import { computed, onMounted } from "vue";
import { useStore } from "vuex";

const store = useStore();

// Computed
const getPaths = computed(() => store.state.modal.listener.paths);

const isDark = computed(() => store.getters["config/isDark"]);

// Functions
const removePath = (idx: number) => store.dispatch("modal/removePath", idx);

const openPath = async (path: string) => await shell.open(path);

onMounted(() => {});
</script>

<template>
  <div class="h-screen w-full overflow-y-auto">
    <h1 class="title text-xl mb-7 pl-8">Monitor Location</h1>

    <DetailCard
      @click="() => openPath(path)"
      class="ml-8 mr-8 dark:text-gray-300 hover:cursor-pointer"
      v-for="(path, idx) in getPaths"
    >
      <template #header>
        <header class="header relative flex items-center p-[3px]">
          <span class="header pl-3 text-sm">Monitoring Path</span>
          <button
            @click.stop="removePath(idx)"
            class="absolute right-6 text-xs hover:text-[#FF0303]"
          >
            Remove
          </button>
        </header>
      </template>

      <template #content>
        <div class="content p-1 flex">
          <Icon
            class="mx-3 mr-4"
            width="70"
            height="70"
            name="monitor"
            :key="(isDark as number)"
            :fill="isDark ? '#b6b6b6' : 'black'"
          />

          <div class="detail flex-grow justify-center flex flex-col">
            <Path :path="path" />
          </div>
        </div>
      </template>
    </DetailCard>
  </div>
</template>
