export type OS = "win32" | "macos" | "auto";

export interface State {
  isDark: boolean;
  titlebar: OS;
  pinNavbar: "pin" | "unpin";
  scanningInterval: number;
  chunks: number;
}
