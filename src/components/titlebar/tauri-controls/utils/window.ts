import { getCurrent } from "@tauri-apps/api/window";
import { ref } from "vue";

export const appWindow = getCurrent();
export const isWindowMaximized = ref(false)

export const minimizeWindow = async () => {
  await appWindow.minimize()
}

export const maximizeWindow = async () => {
  await appWindow.toggleMaximize();
  isWindowMaximized.value = !isWindowMaximized.value;
}

export const fullscreenWindow = async () => {
  const window = appWindow;
  if (window) {
    const fullscreen = await window.isFullscreen();
    await window.setFullscreen(!fullscreen);
  }
}

export const closeWindow = async () => {
  await appWindow.close()
}
