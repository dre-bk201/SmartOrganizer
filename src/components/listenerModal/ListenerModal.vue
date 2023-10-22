<script setup lang="ts">
import Logo from "../common/Logo.vue";
import Switch from "../common/Switch.vue";
import Dropdown from "../common/Dropdown.vue";
import Dot from "../common/Dot.vue";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'


import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from "@/components/ui/tooltip";

import { EditIcon, PlusIcon, TrashIcon, InfoIcon} from "@/icons";

import { computed, reactive, ref, shallowRef } from "vue";
import { TListenerStore } from "../../store";
import clone from "clone";

import { routeOpts, HINTS  } from "../../static";
import { dialog } from "@tauri-apps/api";
import isEqual from "lodash.isequal";
import { isDefault } from "../../utils";

const props = defineProps<{ open: (id:string) => void, close: () => void, radius: number; listener: TListenerStore }>();
const { findById, setActiveId, activeId, deleteListener } = props.listener;

const root = ref<HTMLElement | null>();

let state = reactive(clone(props.listener.activeListener!));
let route = ref("Monitor");
let routeOpt = shallowRef<TDropdownOpts>(routeOpts[0]);

const color = computed(() => (state.enabled ? "bg-[#1EF127]" : "bg-[#DC4343]"));
const distance = computed(() => (state.enabled ? "left-1" : "left-[38px]"));
const hint = computed(() => HINTS[route.value]);
const amount = computed(() => {
  if (route.value == "Monitor") return state.monitors.length;
  if (route.value == "Rule") return state.rules.length;
  if (route.value == "Action") return state.actions.length;
});
const step = computed(() => {
  let views = ["Monitor", "Rule", "Action"];
  let curr_idx = views.indexOf(route.value);
  return {
    prev: views[curr_idx - 1] ?? "",
    next: views[curr_idx + 1] ?? "",
  };
});

function toggleEnable() {
  state.enabled = !state.enabled;
}

function closeDialog() {
  props.close();
  setActiveId();
}

async function closeModal() {
  // if the state is default discard any changes and close modal
  if (isDefault(state)) {
    await deleteListener(state.id);
    return closeDialog();
  }
  // if there are no changes and the state is not default, close modal
  if (isEqual(findById(activeId), state)) return closeDialog();
  (root.value?.querySelector(".unsavedPopupTrigger") as HTMLButtonElement)?.click();
}

function navigateTo(kind: "prev" | "next") {
  if (kind == "prev") route.value = step.value.prev;
  else route.value = step.value.next;
}

async function saveChanges() {
  await props.listener.updateById(state.id, state);
  closeModal();
}

function removePopup() {
  setTimeout(() => {
  (root.value?.querySelector(".deletePopupTrigger") as HTMLButtonElement)?.click();
  }, 300);
}

async function removeListener() {
  await deleteListener(state.id);
  closeDialog();
}

function setOpt(opt: TDropdownOpts | undefined) {
  if (opt) routeOpt.value = opt;
}

async function addNew() {
  if (route.value == "Monitor") {
    let path = await dialog.open({ directory: true, multiple: false });
    if (path?.length) state.monitors.push(path as string);
  }
  if (route.value == "Rule")
    state.rules.push({
      search: "FileName",
      condition: "Includes",
      data: { text: [], size: [0, "B"] },
    });
  if (route.value == "Action")
    state.actions.push({
      action: "DELETE",
      path: "",
    });
}

defineExpose({
  remove: removePopup
})

</script>

<template>
  <main ref="root" class="flex adaptive--dark">
    <AlertDialog>
      <AlertDialogTrigger as="button" class="unsavedPopupTrigger" />
      <AlertDialogContent class="border-none">
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            This action cannot be undone. You will permanently lose any changes made to this listener.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel class="hover:border-white border-transparent hover:text-white">Cancel</AlertDialogCancel>
          <AlertDialogAction @click="closeDialog">Continue</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>

    <div class="adaptive--dark w-[260px] flex-shrink-0 flex flex-col gap-8">
      <section class="px-10">
        <Logo :isPinned="false" />
      </section>

      <section class="pl-2 flex justify-center">
        <img src="../../assets/img/listenerdetail.png" />
      </section>

      <section>
        <section
          class="mb-3 pl-2 flex justify-center shadow-md shadow-darker relative items-center rounded-md dark:text-white adaptive--dark--invert dark:!bg-[#424349] common-mx py-3">
          <EditIcon class="z-10 absolute left-2 dark:text-white" />
          <input autofocus v-model="state.title" class="ml-6 bg-transparent w-full outline-none" type="text"
            placeholder="Organizer Name" />
        </section>

        <section
          class="pl-3 flex justify-start shadow-md shadow-darker relative items-center rounded-md adaptive--dark--invert dark:!bg-[#424349] common-mx p-3">
          <input v-model="state.deep" class="mr-3 scale-150 outline-none" type="checkbox" />
          <span class="dark:text-gray-100">Deep Search</span>
        </section>
      </section>

      <section class="text-white">
        <Switch @click="toggleEnable" :state="state.enabled" first="ENABLE" second="DISABLE">
          <div class="switch--thumb adaptive--dark" :class="[distance]">
            <Dot :class="[color, '!border-[0px] !w-[12px] !h-[12px]']" />
          </div>
        </Switch>
      </section>
    </div>

    <div class="flex-grow flex flex-col dark:text-gray-200 px-8 py-5 adaptive--dark--invert"
      :style="{ borderTopLeftRadius: radius + 'px' }">
      <section class="xl:min-w-[800px] xl:mx-auto text-[#6C6E76] flex items-center">
        <span class="text-2xl">Listener</span>
        <span class="mx-2 text-2xl">/</span>

        <Dropdown v-model="route" :options="routeOpts">
          <template #label="{ opt }">
            <span class="text-2xl text-white">
              {{ setOpt(opt) }}
              <!--TODO: Updates current opt-->
              {{ opt?.label }}
            </span>
          </template>
        </Dropdown>

        <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <InfoIcon class="ml-5 text-xl" />
              </TooltipTrigger>
              <TooltipContent class="bg-primary max-w-[15rem] break-words" side="right">
                {{hint}}
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
      </section>

      <section class="xl:min-w-[800px] xl:mx-auto flex flex-col flex-grow gap-3">
        <section class="flex justify-between mt-8 items-center">
          <div class="flex items-center gap-3">
            <button @click="addNew"
              class="flex font-bold items-center gap-1 pr-4 pl-1 py-2 bg-primary text-black rounded-md">
              <PlusIcon class="text-xl" />
              Add New
            </button>
            <span class="text-xl">{{ amount }} {{ route + "(s)" }}</span>
          </div>


          <AlertDialog>
            <AlertDialogTrigger as="button"
              class="deletePopupTrigger dark:bg-[#424349] text-[#DC4343] py-2 px-3 rounded-md flex items-center gap-2">
              <TrashIcon />
              Delete Listener
            </AlertDialogTrigger>
            <AlertDialogContent class="border-none">
              <AlertDialogHeader>
                <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                <AlertDialogDescription>
                  This action cannot be undone. This will permanently delete this listener without the possibilty of retrieval.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel class="hover:border-white border-transparent hover:text-white">Cancel</AlertDialogCancel>
                <AlertDialogAction @click="removeListener">Continue</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </section>

        <hr class="border-gray-600 mt-2" />

        <div v-if="route == 'Rule'" class="mb-2">
          <label class="items-center gap-2 inline-flex mr-9">
            <input v-model="state.selection" type="radio" name="rules" value="All" />
            <span class="poppins">Apply all of the following</span>
          </label>

          <label class="items-center gap-2 inline-flex mb-3">
            <input v-model="state.selection" type="radio" name="rules" value="Any" />
            <span>Apply any of the following</span>
          </label>

          <hr class="border-gray-600" />
        </div>


        <!-- Content -->
        <div class="flex-grow flex flex-col gap-4 h-0 overflow-y-auto pr-2">
          <component :is="routeOpt.component" v-model="state" />
        </div>

        <hr class="mt-4 border-gray-500" />
        <section class="flex flex-col gap-2">
          <!-- Adding Actions -->
          <div class="flex justify-between items-end">
            <button @click="navigateTo('prev')" v-show="step.prev"
              class="font-[800] text-[1.3rem] text-primary w-fit flex flex-col">
              <span class="text-left text-gray-300 text-xs font-[600]">PREV. STEP</span>
              <span>ADDING {{ step.prev.toUpperCase() + "S" }}</span>
            </button>

            <div />

            <button @click="navigateTo('next')" v-show="step.next"
              class="font-[800] text-[1.3rem] text-primary w-fit flex flex-col">
              <span class="text-right text-gray-300 text-xs font-[600]">NEXT STEP</span>
              <span>ADDING {{ step.next.toUpperCase() + "S" }}</span>
            </button>
          </div>

          <div class="flex justify-end gap-3">
            <button @click="closeModal" class="bg-[#424349] py-2 px-3 rounded-md font-semibold">
              Close w/o saving
            </button>
            <button @click="saveChanges" class="bg-[#424349] py-2 px-3 rounded-md font-semibold">
              Save
            </button>
          </div>
        </section>
      </section>
    </div>
  </main>
</template>

<style scoped>
.common-mx {
  @apply mx-6;
}

.flex-row-x-center {
  @apply flex justify-center;
}

.switch--thumb {
  @apply dark:!bg-[#424349] absolute transition-all px-[13px] py-[12px] rounded-md;
}

.v-enter-active,
.v-leave-active {
  transition: all 0.3s ease;
}

.v-enter-from,
.v-leave-to {
  transform: translateX(-20px);
  opacity: 0;
}
</style>
