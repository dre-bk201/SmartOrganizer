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


<template>
  <div
    class="absolute hover:cursor-pointer flex p-4 flex-col z-[2] bottom-0 gap-4 w-full h-[calc(100%-75px)] adaptive--dark">
    <span @click="close" class="flex dark:bg-darker bg-lighter py-3 pl-3 rounded-md items-center gap-4">
      <ChevronIcon />
      <span>Settings</span>
    </span>

    <section class="flex flex-col gap-4">
      <section>
        <h1>TITLE BAR</h1>

        <div class="p-[5px] border-2 border-[#38AC7A] rounded-xl mb-5 my-3">
          <div class="h-24 flex justify-end items-end bg-[#38AC7A] relative rounded-md ">
            <div class="absolute top-2 flex items-center gap-1 text-sm left-7 scale-125 ">
              <label>
                <input id="MacOS" v-model="titlebarStyle" name="titleBarStyle" value="macos" type="radio">
                Macintosh Style
              </label>
            </div>
            <img class="w-44" :src="macos" />
          </div>
        </div>

        <div class="p-[5px] border-2 border-[#6C8DFF] rounded-xl mb-5">
          <div class="h-24 flex justify-start items-end bg-[#6C8DFF] relative rounded-md ">
            <div class="absolute top-2 flex items-center gap-1 text-sm left-7 scale-125 ">
              <label>
                <input v-model="titlebarStyle" name="titleBarStyle" value="windows" type="radio">
                Windows Style
              </label>
            </div>
            <img class="w-44" :src="win" />
          </div>
        </div>
      </section>

      <div class="w-full h-[1px] bg-gray-400" />

      <section>
        <h2 class="my-5 dark:text-gray-300 text-gray-800">PIN SIDEBAR</h2>
        <div @click="pin = !pin" class="flex justify-center">
          <Switch first="PIN" second="UNPIN" :state="pin">
            <UnlockIcon
              class="dark:bg-[#424349] bg-light text-blue-500 absolute transition-all p-2 box-content rounded-md bg text-lg"
              :class="[pin ? 'left-1' : 'left-[38px]']" />
          </Switch>
        </div>
      </section>
    </section>

  </div>
</template>
