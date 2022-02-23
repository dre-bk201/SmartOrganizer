<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";

const props = defineProps<{ path: string }>();
let dir_len = ref(0);

onMounted(() => {
  invoke("dir_len", { path: props.path }).then(
    (v) => (dir_len.value = <number>v)
  );
});
</script>

<template>
  <span class="font-bold">{{ path }}</span>
  <span>{{ dir_len }} Files or Folders</span>
</template>
