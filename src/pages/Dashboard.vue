<script setup lang="ts">
import Listener from '../components/listener/Listener.vue';
import ContextMenu from '../components/common/ContextMenu.vue';

import EditIcon from "~icons/edit";
import TrashIcon from "~icons/trash";
import Dot from "../components/common/Dot.vue";

import { ListenerStoreKey, ListenerModalKey } from '../store';
import { STATUSCOLORS, defaultListener } from "../static";

import { inject, ref, computed } from 'vue';

import emptystate from "../assets/img/empty-state.png";

const { listener } = inject(ListenerStoreKey)!;
const { remove, open } = inject(ListenerModalKey)!;
const contextMenu = ref<null | InstanceType<typeof ContextMenu>>(null);

let lId = ref("");

const filter = computed(() =>
  listener.listeners.filter(l => l.title.toLowerCase().includes(listener.search.toLowerCase()))
);

const enabled = computed(() => {
  let found = listener.findById(lId.value);
  if (!found) return false;
  return found.enabled;
});

const status = computed(() => {
  let statusMap = {
    ACTIVE: 0,
    INACTIVE: 0,
    DRAFT: 0,
  };

  for (let l of listener.getListeners) {
    let found = statusFromId(l.id);
    switch (found) {
      case "ACTIVE":
        statusMap[found]++;
        break;
      case "INACTIVE":
        statusMap[found]++;
        break;
      case "DRAFT":
        statusMap[found]++;
        break;
    }
  }

  return statusMap;
});

function statusFromId(id: string) {
  let found = listener.listeners.find(lst => lst.id == id);
  if (!found) return null;

  let status = found.enabled ? "ACTIVE" : "INACTIVE";
  if (
    found.monitors.length < 1 ||
    found.rules.length < 1 ||
    found.actions.length < 1
  ) {
    status = "DRAFT";
  }

  return status;
}


function setId(id: string) {
  listener.setActiveId(id);
}

function edit(id: string) {
  setId(id);
  open(id)
}

function deleteListener() {
  contextMenu.value?.close();
  remove(lId.value);
}

async function openAddListenerModal() {
  let l = defaultListener();
  await listener.addListener(l);
  open(l.id);
}

function showContextMenu(id: string) {
  lId.value = id;
  contextMenu.value?.show();
}

</script>

<template>
  <div v-if="listener.listeners.length" class="h-full px-3 flex flex-col">
    <section class="flex gap-8 pt-6 px-3">
      <span v-for="(value, key) in status"
        class="chip p-2 outline dark:outline-light_hl adaptive--dark outline-dark_hl outline-1 rounded-md fg--theme inline-flex justify-center items-center">
        <span :style="{ color: STATUSCOLORS[key] }" class="px-1 text-4xl font-bold"> {{ value }} </span>
        <span class="inline-flex flex-col">
          <span class="text-xs text-primary"> {{ key }} </span>
          <span class="text-lg font-bold text-primary"> LISTENERS </span>
        </span>
      </span>
    </section>

    <section class="my-3 flex items-center px-3 flex-shrink-0">
      <span class="mr-4 text-xl">Listeners</span>
      <span class="flex-grow bg-gray-600 w-full h-[1px]" />
    </section>

    <section class="flex-grow h-0 overflow-y-auto px-3 flex flex-col gap-5">
      <div tabindex="1" id="focusBlank" class="absolute w-0 h-0" />
      <ContextMenu ref="contextMenu">
        <ul class="py-2 px-2 dark:bg-[#424349] shadow-lg rounded-md w-[170px] flex flex-col gap-2">
          <li
            class="px-5 py-3 hover:cursor-pointer hover:bg-darker flex bg-[#303136] text-white overflow-hidden rounded-md items-center gap-4"
            @click="() => { listener.toggleEnabled(lId); contextMenu?.close() }">
            <Dot :class="!enabled ? 'bg-green-400 border-[#1C732F]' : 'bg-[#FC4343] border-[#8C4343]'" />
            {{ enabled ? "Disable" : "Enable" }}
          </li>

          <li
            class="px-5 py-3 hover:cursor-pointer hover:bg-darker flex bg-[#303136] text-white overflow-hidden rounded-md items-center gap-4"
            @click="() => edit(lId)">
            <EditIcon />
            Edit
          </li>

          <li
            class="px-5 py-3 hover:cursor-pointer hover:bg-darker flex bg-[#303136] text-white overflow-hidden rounded-md items-center gap-4"
            @click="() => deleteListener()">
            <TrashIcon class="text-red-500" />
            Delete
          </li>

        </ul>
      </ContextMenu>

      <Listener @click.right.prevent="() => showContextMenu(lst.id)" v-bind="lst" @set-active-id="setId"
        v-for="lst in filter" :key="lst.id" />
    </section>

  </div>

  <div v-else class="h-full text-1xl text-[#50505B] flex flex-col items-center gap-4 justify-center">
    <h3 class="font-poppins text-4xl text-dark dark:text-white">There's no</h3>
    <h1 class="oo">ORGANIZER(S)</h1>
    <img :src="emptystate">

    <p class="w-[70%]">
      Oh, you currently have created no organizer(s). You can do so by clicking the
      <span @click="openAddListenerModal" class="cursor-pointer py-1 px-2 rounded-md text-[#6C8DFF] dark:!bg-[#18181B] adaptive--dark">
        + Add New Listener
      </span>
      &nbsp;in the side bar.
    </p>

    <p class="text-[#838393]">“Getting your files in order one folder at a time”</p>

  </div>
</template>


<style scoped>
.font-poppins {
  font-family: 'Poppins';
  font-style: normal;
}

.oo {
  font-family: 'Poppins';
  font-style: normal;
  font-weight: 900;
  font-size: 52px;
  line-height: 58px;

  text-align: center;
  text-transform: uppercase;

  background: linear-gradient(180deg, #6C8DFF 28.21%, #1B358E 73.08%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}</style>
