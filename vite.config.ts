import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from "path";

import UnpluginIcons from "unplugin-icons/vite";

import { promises as fs } from "fs";

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  plugins: [
    vue(),
    UnpluginIcons({
      compiler: "vue3",
      customCollections: {

        alertcircle: async () =>
          await fs.readFile("./src/assets/img/alert-circle.svg", "utf-8"),

        trash: async () =>
          await fs.readFile("./src/assets/img/trash.svg", "utf-8"),

        settings: async () =>
          await fs.readFile("./src/assets/img/settings.svg", "utf-8"),

        unlock: async () =>
          await fs.readFile("./src/assets/img/unlock.svg", "utf-8"),

        search: async () =>
          await fs.readFile("./src/assets/img/search.svg", "utf-8"),

        statistics: async () =>
          await fs.readFile("./src/assets/img/statistics.svg", "utf-8"),

        plus: async () =>
          await fs.readFile("./src/assets/img/plus.svg", "utf-8"),

        chevron: async () =>
          await fs.readFile("./src/assets/img/chevron.svg", "utf-8"),

        dashboard: async () =>
          await fs.readFile("./src/assets/img/dashboard.svg", "utf-8"),

        edit: async () =>
          await fs.readFile("./src/assets/img/edit.svg", "utf-8"),

        journal: async () =>
          await fs.readFile("./src/assets/img/journal.svg", "utf-8"),

        light: async () =>
          await fs.readFile("./src/assets/img/light.svg", "utf-8"),

        close: async () =>
          await fs.readFile("./src/assets/img/close.svg", "utf-8"),

        maximize: async () =>
          await fs.readFile("./src/assets/img/maximize.svg", "utf-8"),

        minimize: async () =>
          await fs.readFile("./src/assets/img/minimize.svg", "utf-8"),

        logo: async () =>
          await fs.readFile("./src/assets/img/logo.svg", "utf-8"),

        removeCircle: async () =>
          await fs.readFile("./src/assets/img/remove-circle.svg", "utf-8"),

        log_error: async () => await fs.readFile("./src/assets/img/log-error.svg", "utf-8"),

        log_success: async () => await fs.readFile("./src/assets/img/log-success.svg", "utf-8"),

        log_warn: async () => await fs.readFile("./src/assets/img/log-warn.svg", "utf-8"),

        open_modal: async () => await fs.readFile("./src/assets/img/open-modal.svg", "utf-8"),

        filter: async () => await fs.readFile("./src/assets/img/filter.svg", "utf-8"),

        sort: async () => await fs.readFile("./src/assets/img/sort.svg", "utf-8"),

        copy: async () => await fs.readFile("./src/assets/img/copy.svg", "utf-8"),

        stats_folder: async () => await fs.readFile("./src/assets/img/stats_folder.svg", "utf-8"),

        stats_file: async () => await fs.readFile("./src/assets/img/stats_file.svg", "utf-8"),

        info: async () => await fs.readFile("./src/assets/img/info.svg", "utf-8"),
      },

      transform(svg, collection, _) {
        let strokes: string[] = [
          "trash",
          "dashboard",
          "journal",
          "edit",
          "light",
          "unlock",
          "statistics",
          "plus",
          "settings",
          "search",
          "chevron",
          "removeCircle",
          "log_error",
          "log_success",
          "info",
          "log_warn",
          "open_modal",
          "filter",
          "sort",
          "copy",
          "stats_file",
          "stats_folder",
        ];

        let fills = [
          "minimize",
          "close",
          "maximize"
        ]

        if (strokes.includes(collection)) return replace(svg, Replace.Stroke);
        if (fills.includes(collection)) return replace(svg, Replace.Fill)
        if (collection == "logo") return removeWxH(svg);
        return svg;
      },
    })
  ],
});


export enum Replace {
  Stroke,
  Fill,
  Both,
}

export function removeWxH(svg: string) {
  return svg.replace(/ width="\d+"/, " ").replace(/ height="\d+"/, "");
}

export function replace(svg: string, r: Replace): string {
  let adjusted_svg = removeWxH(svg);

  if (r == Replace.Fill)
    adjusted_svg = adjusted_svg.replace(
      /fill="[#\d\w]*"/g,
      "fill='currentColor'"
    );
  else if (r == Replace.Stroke)
    adjusted_svg = adjusted_svg.replace(
      /stroke="[#\d\w]*"/g,
      "stroke='currentColor'"
    );
  else if (r == Replace.Both)
    adjusted_svg = adjusted_svg
      .replace(/fill="[#\d\w]*"/g, "fill='currentColor'")
      .replace(/stroke="[#\d\w]*"/g, "stroke='currentColor'");

  return adjusted_svg;
}
