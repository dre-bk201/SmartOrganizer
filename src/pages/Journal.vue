<script setup lang="ts">
import {
  SearchIcon, LogWarnIcon, OpenModalIcon,
  SortIcon, CopyIcon, CloseIcon
} from "@/icons";

import ContextMenu from "../components/common/ContextMenu.vue";

import { DialogClose } from "radix-vue";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogTrigger,
} from '@/components/ui/dialog';

import { computed, ref, inject, reactive, toRaw, ComputedRef } from "vue";
import { format, formatDistance } from "date-fns";
import { writeText } from "@tauri-apps/api/clipboard";

import { ListenerStoreKey } from "../store";
import { INFO, WARN, SUCCESS, ERROR } from "../static";
import { useStatus } from "../composables";
import { lower } from "../utils";
import { firstBy } from "thenby";

type SortKey = "asc" | "desc";
type FieldKey = "name" | "timestamp" | "level";

const { listener } = inject(ListenerStoreKey)!;
const contextMenu = ref<null | InstanceType<typeof ContextMenu>>(null);

const map: Record<string, string> = {
  WARN: WARN,
  SUCCESS: SUCCESS,
  ERROR: ERROR,
  INFO: INFO,
  ACTIVE: SUCCESS,
  NOT_FOUND: ERROR,
};

let logs = ref<ILog[]>(listener.logs);
let activeLog = ref<null | ILog>(null);
let sorter = reactive({ search: "", field: ["name"] as FieldKey[], sort: "asc" as SortKey })

const findById = listener.findById;

const logStatus = computed(() => {
  if (!activeLog.value) return;
  let found = findById(activeLog.value?.parentId);
  if (!found) return "NOT_FOUND";
  const status = useStatus(found);
  return status.value;
});

const sorted: ComputedRef<ILog[]> = computed(() => {
  let loglist: ILog[] = toRaw(logs.value);
  if (sorter.field.length == 0) return loglist.filter(log => lower(log.name).includes(lower(sorter.search)));

  let sort: IThenBy<ILog>;

  sorter.field.forEach((field, idx) => {
    if (idx == 0) sort = firstBy(field, sorter.sort);
    else sort = sort.thenBy(field, sorter.sort);
  });

  return loglist.sort(sort!).filter(log => lower(log.name).includes(lower(sorter.search)));
});

function openModal(idx: number) {
  activeLog.value = logs.value[idx];
}

function showContextMenu() {
  contextMenu.value?.show();
}

async function copyToClipboard() {
  await writeText(`Log Level: ${activeLog.value?.level}\nTimestamp: ${activeLog.value?.timestamp}\ntuMessage: ${activeLog.value?.message}`);
}
</script>

<template>
  <div class="w-full h-[calc(100vh-50px)] px-5 pt-5 flex flex-col gap-4">
    <section class="relative overflow-x-hidden h-full flex flex-col px-3 rounded-md pb-3 adaptive--dark">
      <section class="sticky top-0 adaptive--dark custom_grid_layout border-b gap-x-3 border-[#424349] font-bold py-2">
        <p>Listener Name</p>
        <p>Action</p>
        <p>Time</p>
        <p>Message</p>
      </section>

      <section class="overflow-y-auto flex-grow overflow-x-hidden px-4">
        <div v-for="(log, idx) in sorted" class="custom_grid_layout items-center py-4">
          <p class="text-ellipsis overflow-hidden whitespace-nowrap">
            <LogWarnIcon :style="{ color: map[log.level] }" class="inline-block" />
            {{ log.name }}
          </p>

          <p :style="{ color: map[log.level] }"
            class="uppercase bg-[#000000] px-2 py-1 font-bold text-sm p-1 rounded w-fit">
            {{ log.action }}
          </p>
          <p class="text-sm">
            <span>{{ format(new Date(log.timestamp), "PPpp") }}</span>
            <span class="ml-2 text-gray-400">({{
              formatDistance(new Date(log.timestamp), new Date(), {
                addSuffix: true,
                includeSeconds: true,
              })
            }})</span>
          </p>

          <div class="flex items-center gap-4 overflow-hidden">
            <p :style="{ color: map[log.level] }"
              class="text-ellipsis text-sm flex-shrink overflow-hidden whitespace-nowrap">
              {{ log.message }}
            </p>

            <Dialog>
              <DialogTrigger as="span" v-tooltip.top="{ value: 'Open modal', class: 'modal' }">
                <OpenModalIcon @click="openModal(idx)" v-tooltip.top="{ value: 'Open modal', class: 'modal' }" />
              </DialogTrigger>
              <DialogContent class="p-0 border-transparent outline-none">
                <div class="rounded-xl h-[620px] relative flex flex-col px-5 py-2 adaptive--darker overflow-hidden">
                  <!-- Header -->
                  <section class="flex justify-between text-white pt-2">
                    <h1 class="text-2xl">View Logs</h1>
                    <section class="flex gap-5 items-center">
                      <Button class="px-2 text-xs" disabled>Undo Action</Button>
                      <DialogClose>
                        <CloseIcon
                          class="hover:cursor-pointer text-md p-[2px] box-content bg-gray-500 rounded-md" />
                      </DialogClose>
                    </section>
                  </section>

                  <hr class="my-3 border-gray-500" />

                  <!-- Content-->
                  <section class="flex flex-col overflow-y-auto pr-4 gap-3 text-white">
                    <h2 class="text-xl dark:text-gray-400">Listener Info</h2>

                    <hr class="my-1 border-gray-500" />

                    <div class="flex justify-between gap-32">
                      <span>Name</span>
                      <span>{{ activeLog?.name }}</span>
                    </div>

                    <div class="flex justify-between gap-32">
                      <span class="whitespace-nowrap">Monitoring Path</span>
                      <span class="break-all">{{ activeLog?.path }}</span>
                    </div>

                    <div class="flex justify-between">
                      <span>Status</span>
                      <span :style="{ color: map[logStatus!] }" class="font-bold">{{
                        logStatus
                      }}</span>
                    </div>

                    <hr class="my-1 border-gray-500" />
                    <h2 class="text-xl dark:text-gray-400">Event Info</h2>
                    <hr class="my-1 border-gray-500" />

                    <div class="flex justify-between gap-32">
                      <span class="whitespace-nowrap">Action performed</span>
                      <span class="text-primary font-bold">{{ activeLog?.action }}</span>
                    </div>

                    <div class="flex justify-between gap-32">
                      <span class="">Timestamp</span>
                      <span class="text-sm break-all">
                        {{ format(new Date(activeLog!.timestamp), "PPpp") }}
                      </span>
                    </div>

                    <hr class="my-1 border-gray-500" />
                    <h2 class="text-xl dark:text-gray-400">Audit Info</h2>
                    <hr class="my-1 border-gray-500" />

                    <div class="flex justify-between">
                      <span class="">Log level</span>
                      <span class="">{{ activeLog?.level }}</span>
                    </div>

                    <div class="flex flex-col gap-2">
                      <span class="">Log Message</span>
                      <div
                        class="relative text-sm px-2 py-4 border border-gray-400 flex flex-col bg-[#424349] rounded-md">
                        <span @click="copyToClipboard"
                          class="hover:cursor-pointer top-1 absolute right-1 text-primary adaptive--dark w-fit text-xs px-[6px] py-[6px] rounded-md flex gap-2">
                          <CopyIcon />
                          COPY TO CLIPBOARD
                        </span>
                        <span>Log Level: {{ activeLog?.level }}</span>
                        <span>Timestamp: {{ activeLog?.timestamp }}</span>
                        <span>Message: {{ activeLog?.message }}</span>
                      </div>
                    </div>
                  </section>
                </div>
              </DialogContent>
            </Dialog>
          </div>
        </div>
      </section>
    </section>

    <ContextMenu ref="contextMenu" class="pt-2 pb-1 px-[10px] bg-red-300 dark:bg-[#424349] shadow-lg rounded-md">
      <div class="flex text-xs items-center justify-between">
        <span class="text-sm font-bold">Sort By</span>
        <div class="flex items-center justify-between gap-2">
          <span @click="sorter.sort = 'asc'" class="flex items-center gap-1">
            <input @mousedown.prevent type="radio" name="sort" value="asc" v-model="sorter.sort" />
            ASC
          </span>

          <span @click="sorter.sort = 'desc'" class="flex items-center gap-1">
            <input @mousedown.prevent type="radio" name="sort" value="desc" v-model="sorter.sort" />
            DSC
          </span>
        </div>
      </div>
      <ul class="py-1 w-[150px] flex flex-col gap-2 text-primary">
        <li class="menu--item">
          <input @mousedown.prevent type="checkbox" value="level" v-model="sorter.field" />
          Log Level
        </li>

        <li class="menu--item">
          <input @mousedown.prevent type="checkbox" value="name" v-model="sorter.field" />Name
        </li>

        <li class="menu--item">
          <input @mousedown.prevent type="checkbox" value="timestamp" v-model="sorter.field" />
          Date
        </li>
      </ul>
    </ContextMenu>

    <section class="flex relative items-center flex-shrink-0  rounded-lg">
      <SearchIcon class="dark:text-gray-300 absolute text-black left-3 top-3 text-lg" />
      <input placeholder="Keyword Search"
        class="pl-10 h-12 outline-primary outline-1 adaptive--dark rounded-md dark:text-gray-200 flex-grow pr-3"
        v-model="sorter.search" type="text" />
      <SortIcon @click="showContextMenu"
        :class="[contextMenu?.isOpen && 'dark:bg-darker', 'absolute right-4  p-[6px] rounded-md box-content']" />
    </section>
  </div>
</template>

<style scoped>
.grid--layout {
  display: grid;
  grid-template-columns: 0.6fr 0.3fr 0.4fr 1fr;
}

.custom_grid_layout {
  display: grid;
  grid-template-columns: minmax(115px, 0.5fr) 80px 270px 1fr;
  /* grid-template-columns: 20% 100px 280px 40px; */
  row-gap: 10px;
}

.col--1 {
  @apply w-[25%] text-left;
}

.col--2 {
  @apply text-left px-3;
}

.col--3 {
  @apply w-[50%] text-left ml-4 bg-red-300;
}

.menu--item {
  @apply py-[7px] pl-2 hover:cursor-pointer hover:bg-darker flex bg-[#303136] text-white overflow-hidden rounded-[4px] items-center gap-2;
}

td {
  @apply py-2;
}
</style>
