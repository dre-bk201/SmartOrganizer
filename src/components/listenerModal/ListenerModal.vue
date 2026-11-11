<script setup lang="ts">
import { reactive, ref, computed } from 'vue';
import clone from 'clone';
import { X } from '@lucide/vue';
import { isListenerComplete } from '../../lib/listenerValidation';
import isEqual from 'lodash.isequal';
import { confirm, open as browse } from '@tauri-apps/plugin-dialog';
import Monitor from './Monitor.vue';
import Rule from './Rule.vue';
import Action from './Action.vue';
import type { TListenerStore } from '../../store';
import { isDefault } from '../../utils';
const props = defineProps<{open:(id:string)=>void; close:()=>void; radius:number; listener:TListenerStore}>();
let state = reactive(clone(props.listener.activeListener!));
const saving = ref(false);
const error = ref('');
const complete = computed(() => isListenerComplete(state));
async function closeModal() {
 if (!isEqual(props.listener.findById(state.id), state) && !isDefault(state)) {
  if (!await confirm('Discard your unsaved listener changes?', {title:'Unsaved changes',kind:'warning'})) return;
 }
 if (isDefault(state)) await props.listener.deleteListener(state.id);
 props.close();
}
async function saveChanges() {
 saving.value=true; error.value='';
 try { if (!complete.value) state.enabled = false; await props.listener.updateById(state.id, clone(state)); props.close(); }
 catch { error.value='Could not save this listener. Please try again.'; }
 finally { saving.value=false; }
}
async function remove() {
 if (await confirm('Delete this listener? Your files will remain unchanged.',{title:'Delete listener',kind:'warning'})) {
  await props.listener.deleteListener(state.id); props.close();
 }
}
async function addFolder() { const path=await browse({directory:true,multiple:false}); if(typeof path==='string' && !state.monitors.includes(path)) state.monitors.push(path); }
function addRule() {state.rules.push({search:'FileName',condition:'Includes',data:{text:[],size:[0,'B']}});}
function addAction() {state.actions.push({action:'MOVE',path:''});}
defineExpose({remove, requestClose:closeModal});
</script>
<template>
<main class="flow-editor">
 <header class="flow-editor-header"><div><h1>Listener settings</h1><p>Choose what to watch and how to organize it.</p></div><button class="dialog-dismiss" aria-label="Close listener settings" @click="closeModal"><X aria-hidden="true"/></button></header>
 <div class="flow-editor-layout"><div class="dialog-name-field"><label for="listener-name">Listener name</label><input id="listener-name" v-model="state.title" placeholder="e.g. Downloads to documents"></div><div class="flow-steps">
 <section class="flow-step"><span class="flow-step-number">1</span><div><h2>Watch a folder</h2><p class="flow-help">Listen for changes in these locations.</p><Monitor v-model="state"/><p v-if="!state.monitors.length" class="flow-empty-inline">No folders yet. Choose the first folder to watch.</p><button class="flow-button" @click="addFolder">Add folder</button><label class="flow-check"><input type="checkbox" v-model="state.deep"> Include subfolders</label></div></section>
 <section class="flow-step"><span class="flow-step-number">2</span><div><h2>Choose what matches</h2><div class="flow-matching"><span>Apply when</span><label><input type="radio" v-model="state.selection" value="Any"> any rule matches</label><label><input type="radio" v-model="state.selection" value="All"> all rules match</label></div><Rule v-model="state"/><p v-if="!state.rules.length" class="flow-empty-inline">Add a rule for a file name, extension, or size.</p><button class="flow-button" @click="addRule">Add rule</button></div></section>
 <section class="flow-step"><span class="flow-step-number">3</span><div><h2>Choose what happens</h2><p class="flow-help">Run these actions on matching files.</p><Action v-model="state"/><p v-if="!state.actions.length" class="flow-empty-inline">Choose an action and its destination.</p><button class="flow-button" @click="addAction">Add action</button></div></section>
 </div></div>
 <footer class="flow-editor-footer"><div><label class="flow-check"><input type="checkbox" v-model="state.enabled"> Enabled</label><p class="flow-help">{{complete ? 'Ready to save' : 'Incomplete listeners are saved as drafts.'}}</p></div><div class="dialog-footer-actions"><button class="flow-button" :disabled="saving" @click="closeModal">Cancel</button><button class="flow-primary" :disabled="saving" @click="saveChanges">{{saving?'Saving…':'Save listener'}}</button></div><p v-if="error" role="alert" class="dialog-save-error">{{error}}</p></footer>
</main>
</template>
