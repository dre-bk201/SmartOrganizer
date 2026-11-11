<script setup lang="ts">
import { SearchIcon, LogWarnIcon, OpenModalIcon, SortIcon, CopyIcon, CloseIcon } from "@/icons";
import { DialogClose, DialogTitle, DialogDescription } from "radix-vue";
import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { computed, ref, inject, reactive } from "vue";
import { format, formatDistance } from "date-fns";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { ListenerStoreKey } from "../store";
import { useStatus } from "../composables";
import { lower } from "../utils";
import { firstBy } from "thenby";

type SortKey = "asc" | "desc";
type FieldKey = "name" | "timestamp" | "level";
const { listener } = inject(ListenerStoreKey)!;
const activeLog = ref<ILog | null>(null);
const copied = ref(false);
const copyError = ref(false);
const sorter = reactive({ search: "", field: ["name"] as FieldKey[], sort: "asc" as SortKey });
const logStatus = computed(() => {
  if (!activeLog.value) return;
  const found = listener.findById(activeLog.value.parentId);
  return found ? useStatus(found).value : "NOT_FOUND";
});
const sorted = computed(() => {
  const loglist = [...listener.logs];
  if (sorter.field.length) {
    let sort: ReturnType<typeof firstBy<ILog>>;
    sorter.field.forEach((field, idx) => {
      sort = idx === 0 ? firstBy(field, sorter.sort) : sort.thenBy(field, sorter.sort);
    });
    loglist.sort(sort!);
  }
  return loglist.filter(log => lower(log.name).includes(lower(sorter.search)));
});
function openModal(log: ILog) {
  activeLog.value = log;
  copied.value = false;
  copyError.value = false;
}
async function copyToClipboard() {
  if (!activeLog.value) return;
  try {
    await writeText(`Log Level: ${activeLog.value.level}\nTimestamp: ${activeLog.value.timestamp}\nMessage: ${activeLog.value.message}`);
    copied.value = true;
    copyError.value = false;
  } catch {
    copyError.value = true;
  }
}
</script>

<template>
  <main class="journal">
    <header class="journal-heading">
      <div><h1>Activity journal</h1><p>Follow each action from your watched folders.</p></div>
      <span class="journal-count">{{ listener.logs.length }} {{ listener.logs.length === 1 ? 'event' : 'events' }}</span>
    </header>
    <section class="journal-toolbar" aria-label="Filter and sort events">
      <label class="journal-search"><SearchIcon aria-hidden="true" /><span class="sr-only">Find a listener</span><input v-model="sorter.search" type="search" placeholder="Find a listener…" /></label>
      <details class="journal-sort">
        <summary><SortIcon aria-hidden="true" /> Sort events</summary>
        <div class="sort-options">
          <fieldset><legend>Direction</legend><label><input v-model="sorter.sort" type="radio" name="journal-sort" value="asc" /> Ascending</label><label><input v-model="sorter.sort" type="radio" name="journal-sort" value="desc" /> Descending</label></fieldset>
          <fieldset><legend>Sort by</legend><label><input v-model="sorter.field" type="checkbox" value="level" /> Log level</label><label><input v-model="sorter.field" type="checkbox" value="name" /> Listener name</label><label><input v-model="sorter.field" type="checkbox" value="timestamp" /> Date</label></fieldset>
        </div>
      </details>
    </section>
    <section class="journal-results" aria-label="Activity events">
      <div v-if="sorted.length" class="journal-table">
        <div class="journal-row journal-columns" aria-hidden="true"><span>Listener / action</span><span>Time</span><span>Message</span><span></span></div>
        <article v-for="log in sorted" :key="log.id" class="journal-row">
          <div class="event-name"><strong>{{ log.name }}</strong><span class="event-action" :data-level="log.level"><LogWarnIcon aria-hidden="true" />{{ log.action }} · {{ log.level }}</span></div>
          <div class="event-time"><time :datetime="log.timestamp">{{ format(new Date(log.timestamp), "PPpp") }}</time><span>{{ formatDistance(new Date(log.timestamp), new Date(), { addSuffix: true, includeSeconds: true }) }}</span></div>
          <p class="event-message" :title="log.message">{{ log.message }}</p>
          <Dialog>
            <DialogTrigger as-child><button class="event-details" type="button" :aria-label="`View event details for ${log.name}`" @click="openModal(log)"><OpenModalIcon aria-hidden="true" /></button></DialogTrigger>
            <DialogContent class="journal-dialog p-0 gap-0" :show-close-button="false">
              <header class="detail-header"><div><DialogTitle class="detail-title">Event details</DialogTitle><DialogDescription class="detail-description">{{ activeLog?.name }}</DialogDescription></div><DialogClose class="event-details" aria-label="Close event details"><CloseIcon aria-hidden="true" /></DialogClose></header>
              <div v-if="activeLog" class="detail-body">
                <section><h2>Listener</h2><dl><div><dt>Name</dt><dd>{{ activeLog.name }}</dd></div><div><dt>Watched folder</dt><dd>{{ activeLog.path }}</dd></div><div><dt>Status</dt><dd>{{ logStatus === 'NOT_FOUND' ? 'Listener no longer exists' : logStatus }}</dd></div></dl></section>
                <section><h2>Event</h2><dl><div><dt>Action performed</dt><dd>{{ activeLog.action }}</dd></div><div><dt>Timestamp</dt><dd>{{ format(new Date(activeLog.timestamp), "PPpp") }}</dd></div><div><dt>Log level</dt><dd>{{ activeLog.level }}</dd></div></dl></section>
                <section><div class="message-heading"><h2>Log message</h2><button class="flow-button copy-button" type="button" @click="copyToClipboard"><CopyIcon aria-hidden="true" />{{ copied ? 'Copied' : 'Copy event' }}</button></div><p class="detail-message">{{ activeLog.message }}</p><p v-if="copied || copyError" class="copy-status" role="status">{{ copyError ? 'Could not copy this event. Try again.' : 'Event details copied to clipboard.' }}</p></section>
              </div>
            </DialogContent>
          </Dialog>
        </article>
      </div>
      <div v-else class="journal-empty"><h2>{{ sorter.search ? 'No matching events' : 'Your activity starts here' }}</h2><p>{{ sorter.search ? 'Try another listener name or clear your search.' : 'Actions from your listeners will appear here as files are organized.' }}</p><button v-if="sorter.search" class="flow-button" type="button" @click="sorter.search = ''">Clear search</button></div>
    </section>
  </main>
</template>

<style scoped>
.journal{display:flex;flex-direction:column;gap:24px;padding:30px;min-width:0;height:100%;color:var(--flow-text)}
.journal-heading{display:flex;justify-content:space-between;align-items:flex-start;gap:16px}.journal-heading h1{font-size:25px;line-height:1.2;letter-spacing:-.025em;font-weight:600;margin:0 0 9px}.journal-heading p,.journal-count{font-size:13px;color:var(--flow-muted)}.journal-count{white-space:nowrap;padding-top:5px;font-variant-numeric:tabular-nums}
.journal-toolbar{display:flex;justify-content:space-between;gap:12px;align-items:center}.journal-search{display:flex;align-items:center;gap:9px;border:1px solid var(--flow-border);border-radius:6px;padding:8px 11px;color:var(--flow-muted);width:260px;max-width:100%}.journal-search:focus-within{outline:2px solid var(--flow-accent);outline-offset:2px}.journal-search input{min-width:0;width:100%;background:transparent;color:var(--flow-text);outline:none;font-size:13px}.journal-search input::placeholder{color:var(--flow-muted)}
.journal-sort{position:relative;font-size:13px}.journal-sort summary{display:flex;align-items:center;gap:7px;cursor:pointer;padding:9px 11px;border:1px solid var(--flow-border);border-radius:6px;white-space:nowrap;list-style:none}.journal-sort summary::-webkit-details-marker{display:none}.journal-sort summary:hover,.journal-sort[open] summary{background:var(--flow-surface)}.journal-sort summary:focus-visible{outline:2px solid var(--flow-accent);outline-offset:3px}.sort-options{position:absolute;right:0;top:calc(100% + 7px);width:200px;z-index:10;background:var(--flow-bg);padding:16px;border:1px solid var(--flow-border);border-radius:8px}.sort-options fieldset+fieldset{margin-top:14px;border-top:1px solid var(--flow-border);padding-top:12px}.sort-options legend{font-weight:600;margin-bottom:7px}.sort-options label{display:flex;gap:8px;align-items:center;padding:5px 0}
.journal-results{min-height:0;overflow:auto;flex:1}.journal-row{display:grid;grid-template-columns:minmax(130px,1fr) minmax(140px,.9fr) minmax(120px,1.4fr) 32px;gap:18px;align-items:center;padding:17px 0;border-bottom:1px solid var(--flow-border);font-size:13px}.journal-columns{font-size:12px;color:var(--flow-muted);padding-top:0;padding-bottom:11px}.event-name,.event-time{display:flex;flex-direction:column;gap:6px;min-width:0}.event-name strong{font-weight:600;overflow-wrap:anywhere}.event-action{display:flex;align-items:center;gap:5px;font-size:11px;color:var(--flow-accent)}.event-action[data-level=ERROR]{color:var(--destructive-text,var(--flow-text));font-weight:600}.event-action[data-level=WARN]{font-weight:600;color:var(--flow-text)}.event-time{font-size:12px;font-variant-numeric:tabular-nums}.event-time span{color:var(--flow-muted)}.event-message{overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:var(--flow-muted)}.event-details{display:grid;place-items:center;width:30px;height:30px;flex-shrink:0;border-radius:5px;color:var(--flow-muted);background:transparent}.event-details:hover{color:var(--flow-accent);background:var(--flow-accent-soft)}
.journal-empty{padding:70px 20px;text-align:center;border-top:1px solid var(--flow-border)}.journal-empty h2{font-size:18px;font-weight:600;margin-bottom:9px}.journal-empty p{font-size:13px;color:var(--flow-muted);max-width:48ch;margin:0 auto 20px}
.journal-dialog{background:var(--flow-bg);color:var(--flow-text);max-height:calc(100vh - 90px);overflow:hidden;border-color:var(--flow-border)}.detail-header{display:flex;justify-content:space-between;gap:16px;padding:22px 24px;border-bottom:1px solid var(--flow-border)}.detail-title{font-size:21px;font-weight:600;letter-spacing:-.02em}.detail-description{font-size:13px;color:var(--flow-muted);margin-top:5px;overflow-wrap:anywhere}.detail-body{padding:24px;overflow:auto;min-height:0}.detail-body section+section{margin-top:26px}.detail-body h2{font-size:14px;font-weight:600;margin-bottom:12px}.detail-body dl>div{display:grid;grid-template-columns:120px minmax(0,1fr);gap:15px;padding:7px 0;font-size:13px}.detail-body dt{color:var(--flow-muted)}.detail-body dd{overflow-wrap:anywhere}.message-heading{display:flex;justify-content:space-between;align-items:center;gap:10px;margin-bottom:12px}.message-heading h2{margin:0}.copy-button{display:flex;align-items:center;gap:6px;padding:6px 9px;font-size:12px}.detail-message{background:var(--flow-surface);padding:14px;border-radius:6px;font-size:13px;white-space:pre-wrap;overflow-wrap:anywhere}.copy-status{font-size:12px;color:var(--flow-muted);margin-top:9px}
@media(max-width:760px){.journal{padding:22px 20px;gap:20px}.journal-row{grid-template-columns:minmax(100px,1fr) minmax(120px,1fr) 30px;gap:12px}.journal-columns span:nth-child(3){display:none}.event-message{grid-column:1 / 3;grid-row:2;white-space:normal;overflow-wrap:anywhere}.journal-row>.event-details{grid-column:3;grid-row:1}.journal-search{width:220px}.journal-row>:last-child{grid-column:3;grid-row:1}.journal-heading{flex-wrap:wrap}}
@media(max-width:450px){.journal{padding:20px 14px}.journal-toolbar{align-items:flex-start;flex-wrap:wrap}.journal-search{width:100%}.detail-header,.detail-body{padding:18px}.detail-body dl>div{grid-template-columns:1fr;gap:3px}}
</style>
