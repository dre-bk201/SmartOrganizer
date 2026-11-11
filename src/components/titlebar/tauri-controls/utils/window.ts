import { getCurrentWindow } from "@tauri-apps/api/window";
import { ref } from "vue";

export const appWindow = getCurrentWindow();
export const isWindowMaximized = ref(false)
const refreshMaximized = async () => {
  isWindowMaximized.value = await appWindow.isMaximized();
};
void refreshMaximized().catch(console.error);
void appWindow.onResized(() => { void refreshMaximized().catch(console.error); }).catch(console.error);

export const minimizeWindow = async () => {
  await appWindow.minimize()
}

export const maximizeWindow = async () => {
  await appWindow.toggleMaximize();
  await refreshMaximized();
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
