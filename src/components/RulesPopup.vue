<script lang="ts" setup>
import Modal from "./Modal.vue";
import DetailCard from "./DetailCard.vue";
import { useStore } from "vuex";
import { onMounted, ref } from "vue";
import {
  FILECONTENT,
  FILEEXTENSION,
  FILENAME,
  FILESIZE,
  FOLDERNAME,
  PATHNAME,
} from "../utils/index";

interface Props {
  idx?: number;
  searchType?: string;
  condition?: string;
  text?: string;
}

const props = defineProps<Props>();
const store = useStore();

// Variables
const state = ref<Props>(JSON.parse(JSON.stringify(props)));
const sectionRef = ref<HTMLDivElement | null>();

// Functions
const closeModal = () => store.dispatch("modal/setCurrentRule", {});

const saveModal = () => {
  store.dispatch("modal/updateRuleByIdx", state.value);
  closeModal();
};

onMounted(() => {
  state.value = JSON.parse(JSON.stringify(props));
});
</script>

<template>
  <Modal ref="modalRef">
    <template #modal>
      <div
        class="dialog w-[80%] h-[80%] max-h-[500px] max-w-[800px] relative rounded-xl bg-l_primary dark:bg-d_primary p-8 pt-5">
        <div
          class="left--block bg-l_primary z-30 rounded-bl-lg rounded-tl-lg dark:bg-d_primary w-[50%] h-full absolute left-0 top-0" />
        <div
          class="right--block rounded-tr-lg z-30 rounded-br bg-l_primary dark:bg-d_primary w-[50%] h-full absolute right-0 top-0" />
        <div class="container absolute flex justify-end right-8">
          <button @click="closeModal" class="mr-5 bg-[#F3F3F3] dark:bg-d_secondary px-3 py-1 rounded-md">
            Close
          </button>
          <button @click="saveModal" class="bg-[#FF3B76] text-white px-3 py-1 rounded-md">
            Save
          </button>
        </div>
        <header class="text-2xl mb-6">Create/Edit Rules</header>
        <section ref="sectionRef" class="flex flex-col">
          <DetailCard>
            <template #header>
              <span class="p-3"> Type of Search </span>
            </template>

            <template #content>
              <div class="options--grid p-2">
                <div class="m-1">
                  <input class="mr-2" :id="FILENAME" type="radio" name="searchType" :value="FILENAME"
                    v-model="state.searchType" />
                  <label class="text-lg" for="File Name">File Name</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" :id="FILEEXTENSION" type="radio" name="searchType" :value="FILEEXTENSION"
                    v-model="state.searchType" />
                  <label class="text-lg" for="File Extension">File Extension</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" :id="FOLDERNAME" type="radio" name="searchType" :value="FOLDERNAME"
                    v-model="state.searchType" />
                  <label class="text-lg" for="Folder Name">Folder Name</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" :id="FILESIZE" type="radio" name="searchType" :value="FILESIZE" disabled
                    v-model="state.searchType" />
                  <label class="text-lg" for="File Size">File Size</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" :id="FILECONTENT" type="radio" name="searchType" :value="FILECONTENT"
                    v-model="state.searchType" />
                  <label class="text-lg" for="File Content">File Content</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" :id="PATHNAME" type="radio" name="searchType" :value="PATHNAME"
                    v-model="state.searchType" />
                  <label class="text-lg" for="Path Name">Path Name </label>
                </div>
              </div>
            </template>
          </DetailCard>

          <DetailCard>
            <template #header>
              <span class="p-3"> Conditions </span>
            </template>

            <template #content>
              <div class="options--grid p-2">
                <div class="m-1">
                  <input class="mr-2" id="Includes" type="radio" value="Includes" v-model="state.condition"
                    name="condition" />
                  <label class="text-lg" for="Includes">Includes</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" id="Not Includes" type="radio" value="Not Includes" v-model="state.condition"
                    name="condition" />
                  <label class="text-lg" for="Not Includes">Not Includes</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" id="Exact Match" type="radio" name="condition" value="Exact Match"
                    v-model="state.condition" />
                  <label class="text-lg" for="Exact Match">Exact Match</label>
                </div>

                <div class="m-1">
                  <input class="mr-2" id="Is Not" type="radio" name="condition" value="Is Not"
                    v-model="state.condition" />
                  <label class="text-lg" for="Is Not">Is Not</label>
                </div>
              </div>
            </template>
          </DetailCard>
        </section>

        <input class="indent-8 w-full h-10 rounded-md outline-none bg-l_white dark:bg-d_secondary" type="text"
          v-model="state.text" placeholder="Text to filter for" />
      </div>
    </template>
  </Modal>
</template>

<style scoped>
button {
  box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.25);
  border-radius: 4px;
}
</style>
