<script lang="ts" setup></script>

<template>
  <div
    class="w-screen h-screen z-10 fixed left-0 top-0 overflow-hidden rounded-md"
  >
    <div class="bg--filter w-full h-full absolute" />
    <div class="relative center w-full h-full">
      <slot name="modal">
        <slot name="header">This is the header</slot>
      </slot>
    </div>
  </div>
</template>
