<script setup lang="ts">
import { computed, ref } from "vue";
import { TSettingStore } from "../../store";

const { settingStore } = defineProps<{
  settingStore: TSettingStore
}>()

let pin = ref(settingStore.pin);

const radius = computed(() => settingStore.rounded);
const width = computed(() => pin.value ? '90px' : '240px');

</script>

<template>
  <div class="relative h-screen flex flex-col overflow-hidden">
    <div class="flex flex-col h-full">
      <span class="">
        <!-- Titlebar -->
        <slot name="titlebar"></slot>
      </span>

      <div class="flex flex-grow relative adaptive--dark">
        <slot name="listenerModal" :radius="radius"></slot>

        <section class="transition-all flex-shrink-0" :style="{ width }">
          <!-- Navbar -->
          <slot name="navbar" :adjust-width="(p: boolean) => (pin = p)"></slot>
        </section>

        <section class="adaptive--darker flex flex-col flex-grow" :style="{ borderTopLeftRadius: radius + 'px' }">
          <!-- Content -->
          <slot></slot>
        </section>

      </div>

      <section class="">
        <slot name="modal"></slot>
      </section>
    </div>
  </div>
</template>
