import { defaultListener } from "../static";
import isEqual from "lodash.isequal";
import omit from "lodash.omit";

export const sclone = structuredClone;

export function lower(a: string) {
  return a.toLowerCase();
}

export function isDefault(listener: IListener) {
  return isEqual(omit(listener, "id", "created"), omit(defaultListener(), "id", "created"));
}

export function unitMap(val: string, kind: "" | "metric" = ""): string {
  let returnv: string = "";

  if (kind == "metric")
    switch (val as TUnitOpts) {
      case "B":
        return "Byte(s)";
      case "KB":
        return "Kilobyte(s)";
      case "MB":
        return "Megabyte(s)";
      case "GB":
        return "Gigabyte(s)";
      case "TB":
        return "Terabyte(s)";
    }
  else
    switch (val) {
      case "Byte(s)":
        returnv = "B";
        break;
      case "Kilobyte(s)":
        returnv = "KB";
        break;
      case "Megabyte(s)":
        returnv = "MB";
        break;
      case "Gigabyte(s)":
        returnv = "GB";
        break;
      case "Terabyte(s)":
        returnv = "TB";
    }

  return returnv;
}

export function stringToColour(str: string) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }
  let colour = "#";
  for (let i = 0; i < 3; i++) {
    let value = (hash >> (i * 8)) & 0xff;
    colour += ("00" + value.toString(16)).substr(-2);
  }

  return colour;
}
