import { ref, computed, onMounted, onUnmounted } from 'vue';

export function useMouse() {
  const x = ref(0);
  const y = ref(0);

  function update(event: MouseEvent) {
    x.value = event.pageX;
    y.value = event.pageY;
  }

  onMounted(() => window.addEventListener('mousemove', update));
  onUnmounted(() => window.removeEventListener('mousemove', update));

  return { x, y };
}


export function useStatus(listener: IListener) {
  return computed(() => {
    let listenerStatus = listener.enabled ? "ACTIVE" : "INACTIVE";
    if (
      listener.monitors.length < 1 ||
      listener.rules.length < 1 ||
      listener.actions.length < 1 ||
      listener.selection.length < 1
    )
      listenerStatus = "DRAFT";

    return listenerStatus;
  })
}
