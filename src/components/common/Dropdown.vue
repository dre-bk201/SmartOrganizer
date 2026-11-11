<script setup lang="ts">
import { watch } from 'vue';
const props=defineProps<{modelValue:string;options:TDropdownOpts[];disabled?:{state:string;value:string[]}}>();
const emit=defineEmits(['update:modelValue']);
watch(()=>props.disabled?.state,()=>{if(props.disabled?.value.includes(props.modelValue)){const option=props.options.find(o=>!props.disabled?.value.includes(o.value));if(option)emit('update:modelValue',option.value);}});
</script>
<template><select class="flow-select" :aria-label="options.map(o=>o.label).join(', ')" :value="modelValue" @change="emit('update:modelValue',($event.target as HTMLSelectElement).value)"><option v-for="option in options" :key="option.value" :value="option.value" :disabled="disabled?.value.includes(option.value)">{{option.label}}</option></select></template>
<style scoped>.flow-select{padding:8px 28px 8px 10px;background:var(--flow-bg);color:var(--flow-text);border:1px solid var(--flow-border);border-radius:6px;font-size:13px;max-width:100%}</style>
