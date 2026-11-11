import { type, type OsType } from "@tauri-apps/plugin-os";
export function getOsType(): Promise<OsType> {
  return Promise.resolve(type());
}
