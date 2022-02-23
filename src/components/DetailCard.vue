<script lang="ts" setup></script>

<template>
  <div class="detail--card mb-8 bg-l_white dark:bg-d_secondary rounded-md">
    <slot name="header">Header here</slot>
    <div class="h-[1px] bg-l_secondary" />
    <slot name="content">Content here</slot>
  </div>
</template>
