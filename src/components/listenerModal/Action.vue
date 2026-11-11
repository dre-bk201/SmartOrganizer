<script setup lang="ts">
import { ref, useModel } from 'vue';
import { FolderOpen, Trash2, ArrowRight, AlertTriangle } from '@lucide/vue';
import { ALLOWED } from '../../static';
import * as dialog from '@tauri-apps/plugin-dialog';
const props = defineProps<{modelValue:IListener}>();
const proxy = useModel(props,'modelValue');
const pending = ref<number | null>(null);
const errors = ref<Record<number,string>>({});
const options: {value:TActionOpts;label:string;hint:string}[] = [
 {value:'MOVE',label:'Move',hint:'Move matching files into another folder.'},
 {value:'COPY',label:'Copy',hint:'Keep the originals and copy them into another folder.'},
 {value:'RENAME',label:'Rename',hint:'Choose the new filename and location.'},
 {value:'DELETE',label:'Move to trash',hint:'Send matching files to the recycle bin or trash.'},
 {value:'UNLINK',label:'Delete permanently',hint:'Permanently remove matching files. This cannot be undone.'}
];
function allowed(index:number,value:TActionOpts) {
 if(proxy.value.actions[index].action===value)return true;
 return proxy.value.actions.every((item,i)=>i===index || (ALLOWED[item.action].includes(value) && ALLOWED[value].includes(item.action)));
}
function change(index:number,event:Event) {
 const value=(event.target as HTMLSelectElement).value as TActionOpts;
 const item=proxy.value.actions[index];
 const sameFolderKind=['MOVE','COPY'].includes(item.action) && ['MOVE','COPY'].includes(value);
 proxy.value.actions[index]={action:value,path:sameFolderKind?item.path:''};
 delete errors.value[index];
}
async function choose(index:number) {
 const item=proxy.value.actions[index];pending.value=index;delete errors.value[index];
 try {
  const path=item.action==='RENAME' ? await dialog.save({title:'Choose new filename',defaultPath:item.path || undefined}) : await dialog.open({title:'Choose destination folder',directory:true,multiple:false,defaultPath:item.path || undefined});
  if(typeof path==='string' && proxy.value.actions[index]===item)item.path=path;
 } catch { errors.value[index]='Could not open the picker. Please try again.'; }
 finally {pending.value=null;}
}
function remove(index:number) {proxy.value.actions.splice(index,1);errors.value={};}
</script>
<template>
 <div class="action-list">
  <section v-for="(item,index) in proxy.actions" :key="index" class="action-row" :aria-label="`Action ${index+1}`">
   <div class="action-row-heading"><label :for="`action-type-${index}`">Action {{index+1}}</label><button type="button" class="flow-icon-button" :disabled="pending!==null" :aria-label="`Remove action ${index+1}`" @click="remove(index)"><Trash2/></button></div>
   <div class="action-operation"><ArrowRight aria-hidden="true"/><select :id="`action-type-${index}`" :value="item.action" :disabled="pending!==null" @change="change(index,$event)"><option v-for="option in options" :key="option.value" :value="option.value" :disabled="!allowed(index,option.value)">{{option.label}}</option></select><span>{{options.find(option=>option.value===item.action)?.hint}}</span></div>
   <div v-if="!['DELETE','UNLINK'].includes(item.action)" class="action-destination"><label :for="`action-path-${index}`">{{item.action==='RENAME'?'New filename':'Destination folder'}}</label><div class="destination-control"><input :id="`action-path-${index}`" :value="item.path" readonly :placeholder="item.action==='RENAME'?'Choose a new filename…':'No folder selected'" :title="item.path"><button type="button" class="flow-button" :disabled="pending!==null" @click="choose(index)"><FolderOpen aria-hidden="true"/>{{pending===index?'Opening…':item.path?'Change…':'Browse…'}}</button></div></div>
   <p v-if="item.action==='UNLINK'" class="action-warning"><AlertTriangle aria-hidden="true"/>Matching files and folders will be permanently deleted.</p>
   <p v-if="errors[index]" role="alert" class="action-warning">{{errors[index]}}</p>
  </section>
 </div>
</template>
<style scoped>
.action-list{display:flex;flex-direction:column;gap:12px}.action-row{border:1px solid var(--flow-border);border-radius:10px;padding:14px 16px;background:var(--acrylic-well,var(--flow-surface))}.action-row-heading{display:flex;align-items:center;justify-content:space-between;margin-bottom:8px;font-size:12px;color:var(--flow-muted)}.action-operation{display:flex;align-items:center;gap:10px}.action-operation>svg{width:17px;height:17px;flex-shrink:0;color:var(--flow-accent)}.action-operation select{padding:9px 30px 9px 12px;border:1px solid var(--flow-border);border-radius:6px;background:var(--flow-bg);color:var(--flow-text);min-width:160px;font-weight:600}.action-operation>span{font-size:12px;line-height:1.5;color:var(--flow-muted)}.action-destination{margin-top:16px}.action-destination>label{font-size:12px;color:var(--flow-muted);display:block;margin-bottom:7px}.destination-control{display:flex;gap:8px}.destination-control input{min-width:0;flex:1;border:1px solid var(--flow-border);border-radius:6px;padding:9px 12px;color:var(--flow-text);background:var(--flow-bg);text-overflow:ellipsis}.destination-control button{display:flex;align-items:center;gap:7px}.destination-control svg{width:16px;height:16px}.action-warning{display:flex;gap:8px;font-size:12px;margin:14px 0 0;color:var(--flow-accent)}.action-warning svg{width:16px;height:16px;flex-shrink:0}@media(max-width:600px){.action-operation{flex-wrap:wrap}.action-operation>span{flex-basis:100%}.action-operation select{flex:1}.destination-control{flex-wrap:wrap}.destination-control input{flex-basis:100%}}
</style>
