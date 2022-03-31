<script lang="ts" setup>
import ActionItem from "../ActionItem.vue";

import { computed } from "vue";
import { useStore } from "vuex";
import { shell } from "@tauri-apps/api";

const store = useStore();

const getActions = computed(() => {
  return store.state.modal.listener.actions;
});

const openPath = async (path: string) => await shell.open(path);
</script>

<template>
  <div class="h-screen w-full flex flex-col">
    <h1 class="title text-xl mb-7 pl-8">Actions</h1>
    <ActionItem
      @click="() => openPath(action[1])"
      v-for="(action, idx) in getActions"
      :action="action"
      :idx="idx"
    />
  </div>
</template>
