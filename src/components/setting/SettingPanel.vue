<script setup lang="ts">
//@ts-nocheck
import ChevronIcon from "~icons/chevron";
import UnlockIcon from "~icons/unlock";

import Switch from "../common/Switch.vue";

import macos from "../../assets/img/macos.svg";
import win from "../..//assets/img/windows.svg";

import { SettingStoreKey } from "../../store";
import { computed, inject } from "vue";
import { TitlebarStyle } from "../../models";

const { settings } = inject(SettingStoreKey)!;

defineProps<{ close: () => void }>();

const titlebarStyle = computed({
  get: () => (settings.titlebarStyle),
  set(titlebarStyle: TitlebarStyle) { settings.setTitlebarStyle(titlebarStyle) }
})

const pin = computed({
  get: () => (settings.pin),
  set(v: boolean) { settings.setPin(v); }
});
</script>


<template><section class="flow-settings" aria-label="Settings"><header><h2>Settings</h2><button class="flow-button" @click="close">Close</button></header><fieldset><legend>Window controls</legend><label><input v-model="titlebarStyle" name="titlebar" type="radio" value="windows"> Windows</label><label><input v-model="titlebarStyle" name="titlebar" type="radio" value="macos"> macOS</label></fieldset><label class="flow-check"><input type="checkbox" v-model="pin"> Compact sidebar</label></section></template><style scoped>.flow-settings{position:absolute;inset:0;z-index:20;background:var(--flow-surface);padding:18px;color:var(--flow-text);overflow:auto}.flow-settings header{display:flex;align-items:center;justify-content:space-between;gap:8px;margin-bottom:28px}.flow-settings h2{font-size:18px;font-weight:600}.flow-settings fieldset{display:flex;flex-direction:column;gap:14px}.flow-settings legend{font-weight:600;margin-bottom:16px}.flow-settings label{font-size:13px}.flow-check{margin-top:30px}</style>
