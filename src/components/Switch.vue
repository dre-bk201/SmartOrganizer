<script lang="ts" setup>
import { computed } from "vue";
import { useStore } from "vuex";

const store = useStore();

const isActive = computed(() => store.state.modal.listener.enabled);

const toggleActive = () => {
  store.dispatch("modal/toggleActive");
};
</script>

<template>
  <div
    @click="toggleActive"
    class="switch h-9 w-16 bg-[#505050] relative rounded-full hover:cursor-pointer"
  >
    <div
      :class="`fill bg-[#809CFF] h-9 rounded-full absolute ${
        isActive ? 'w-full' : 'w-9'
      }`"
    >
      <div :class="`knob`">
        <div
          :class="`indicator w-2 h-2 rounded-full shadow ${
            isActive
              ? 'bg-[#5BFF58] shadow-[#5BFF58]'
              : 'bg-[#ff3b76] shadow-[#ff3b76]'
          }`"
        />
      </div>
    </div>
  </div>
</template>
