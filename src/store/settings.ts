//@ts-nocheck
import { defineStore } from "pinia";
import { InjectionKey, ComputedRef } from "vue";
import type { TitlebarStyle } from "../models";
import { invoke } from "@tauri-apps/api";

export const SETTINGSTORE = "settingStore";

export type AppTheme = "dark" | "light";

export const useSettingStore = defineStore(SETTINGSTORE, {
  state: () => ({
    pin: true,
    titlebarStyle: "windows" as TitlebarStyle,
    theme: "dark" as AppTheme,
    rounded: 15,
  }),

  actions: {
    setState(state) {
      this.$patch(state)
      console.log(this.$state)
    },

    async toggleTheme() {
      let theme = this.theme == "dark" ? "light" : "dark";

      if (theme === 'dark') {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }

      let saved = await invoke("save_settings", {
        settings: {
          pin: this.pin,
          titlebarStyle: this.titlebarStyle,
          theme: theme,
          rounded: this.rounded
        }
      })

      if (!saved) return;

      this.theme = theme;
    },

    async setTitlebarStyle(style: TitlebarStyle) {
      let saved = await invoke("save_settings", {
        settings: {
          pin: this.pin,
          titlebarStyle: style,
          theme: this.theme,
          rounded: this.rounded
        }
      })

      if (!saved) return;
      this.titlebarStyle = style;
    },

    async setTheme(theme: AppTheme) {
      let saved = await invoke("save_settings", {
        settings: {
          pin: this.pin,
          titlebarStyle: this.titlebarStyle,
          theme,
          rounded: this.rounded
        }
      })

      if (!saved) return;
      this.theme = theme;
    },

    async setPin(pin: boolean) {
      let saved = await invoke("save_settings", {
        settings: {
          pin,
          titlebarStyle: this.titlebarStyle,
          theme: this.theme,
          rounded: this.rounded
        }
      })

      if (!saved) return;
      this.pin = pin;
    },
  },

  getters: {
    isDark(state) {
      return state.theme == "dark";
    },
  },
});

export type TSettingStore = ReturnType<typeof useSettingStore>;
export const SettingStoreKey = Symbol() as InjectionKey<{
  settings: TSettingStore;
}>;
export const ThemeKey = Symbol() as InjectionKey<{
  isDark: ComputedRef<boolean>;
  theme: ComputedRef<AppTheme>;
}>;
