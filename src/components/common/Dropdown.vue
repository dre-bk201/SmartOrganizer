<script setup lang="ts">
import { watch } from 'vue';
import { computed, ref } from 'vue';

import ChevronIcon from "~icons/chevron";

interface Props {
  modelValue: string;
  options: TDropdownOpts[];
  disabled?: { state: string, value: string[] }
}

const props = defineProps<Props>();
const emits = defineEmits(["update:modelValue"]);

const root = ref<HTMLElement | null>(null);

watch(() => props.disabled?.state, () => {
  if (!props.disabled) return
  let disabled = props.disabled.value;

  if (disabled.includes(props.modelValue)) {
    let opts = props.options.filter(opt => !disabled.includes(opt.value));
    emits("update:modelValue", opts[0].value);
  }
});

let isOpen = ref(false);

const currentOpt = computed(() => {
  return props.options.find(opt => opt.value == props.modelValue);
});

function focusIn() {
    isOpen.value = true;
}

function focusOut() {
    isOpen.value = false;
    root.value?.blur();
}

function selectOpt(option: string, disabled: boolean | undefined) {
  if (disabled) return;
  emits('update:modelValue', option);
  focusOut()
}

</script>

<template>
  <div ref="root" class="relative" tabindex="1" @focusin="focusIn" @focusout="focusOut">
    <div class="hover:cursor-pointer w-fit flex items-center justify-center gap-2">
      <slot name="label" :opt="currentOpt">
        <span class="text-[#925EFF] leading-none">{{ currentOpt?.label }}</span>
      </slot>

      <slot name="icon">
        <ChevronIcon
          class="w-[7px] h-[7px] text-2xl box-content p-[3px] shadow-xl rounded-[4px] bg-[#C2CEDA] text-[#242429] rotate-[270deg]" />
      </slot>

      <slot name="item">
      </slot>
    </div>
    <Transition>
      <ul v-if="isOpen"
        class="left-2 mt-2 absolute z-10 shadow-2xl w-fit flex flex-col rounded-[5px] gap-2 dropdown dark:bg-[#424349] p-2">
        <li
          :class="[option.value == modelValue && 'hidden', disabled?.value.includes(option.value) && 'disabled', 'dark:bg-[#303136] bg-light text-left whitespace-nowrap py-1 px-3 rounded-[3px] text-[#925EFF] hover:cursor-pointer']"
          @click="selectOpt(option.value, disabled?.value.includes(option.value))" v-for="option in options">
          {{ option.label }}
        </li>
      </ul>
    </Transition>
  </div>
</template>

<style scoped>
.disabled {
  @apply opacity-50 hover:cursor-default;
}

.v-enter-active,
.v-leave-active {
  transition: opacity 0.3s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
