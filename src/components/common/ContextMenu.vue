<script setup lang="ts">
import { ref } from "vue";
import { useMouse } from "../../composables";
const { x, y } = useMouse();

const contextMenuRef = ref<null | HTMLElement>(null);
const focusBlankElement = document.getElementById("focusBlank");

const isFocused = ref(false);

function focus() {
  isFocused.value = true;
  focusBlankElement?.blur();
}

function unfocus() {
  isFocused.value = false;
  focusBlankElement?.focus();
}

function show() {
  if (!contextMenuRef.value) return
  const pos = contextMenuRef.value.getBoundingClientRect();

  contextMenuRef.value.focus();

  if (x.value + pos.width <= window.innerWidth)
    contextMenuRef.value.style.left = `${x.value + 10}px`;
  else
    contextMenuRef.value.style.left = `${window.innerWidth - (pos.width * 1.1)}px`;

  if (y.value + pos.height <= window.innerHeight)
    contextMenuRef.value.style.top = `${y.value - pos.height * 0.25}px`;
  else
    contextMenuRef.value.style.top = `${y.value - pos.height * 1.2}px`;
}

defineExpose({
  show,
  close: unfocus,
  isOpen: isFocused
});
</script>

<template>
  <section tabindex="1" @focus="focus" @focusout="unfocus"
    :class='[!isFocused && "opacity-0 z-[-1]", "focus:outline-none transition-all rounded-md fixed"]'
    ref="contextMenuRef">
    <slot></slot>
  </section>
</template>
