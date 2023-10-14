import Monitor from "../components/listenerModal/Monitor.vue";
import Action from "../components/listenerModal/Action.vue";
import Rule from "../components/listenerModal/Rule.vue";

import { nanoid } from "nanoid";

export const WARN = "#FF8E3C";
export const SUCCESS = "#1EF127";
export const ERROR = "#DC4343";
export const INFO = "#6C8DFF";

export const defaultListener = (): IListener => ({
  id: nanoid(),
  created: (new Date()).toUTCString(),
  enabled: false,
  monitors: [],
  selection: "Any",
  deep: false,
  title: "",
  actions: [],
  rules: [],
});

export const ALLOWED = {
  COPY: ["MOVE", "DELETE", "RENAME", "UNLINK", "COPY"],
  MOVE: ["COPY"],
  RENAME: ["COPY"],
  UNLINK: ["COPY"],
  DELETE: ["COPY"],
};

export const STATUSCOLORS: Record<string, string> = {
  DRAFT: WARN,
  INACTIVE: "#E5C36A",
  ACTIVE: SUCCESS,
};

export const HINTS: Record<string, string> = {
  Monitor: "`Monitors` are the paths that are subscribed to for events",
  Rule: "`Rules` are a set of conditions that must apply to a file/folder detected by an event",
  Action:
    "`Actions` are the list of actions that should take place if a file/folder matches a `Rule`",
};

export const routeOpts: TDropdownOpts[] = [
  {
    label: "Monitors",
    value: "Monitor",
    component: Monitor,
  },
  {
    label: "Rules",
    value: "Rule",
    component: Rule,
  },
  {
    label: "Actions",
    value: "Action",
    component: Action,
  },
];

export const sizeOpts: TDropdownOpts[] = [
  {
    value: "B",
    label: "Byte(s)",
  },
  {
    value: "KB",
    label: "Kilobyte(s)",
  },
  {
    value: "MB",
    label: "Megabyte(s)",
  },
  {
    value: "GB",
    label: "Gigabyte(s)",
  },
];

export const conditionOpts: TDropdownOpts[] = [
  {
    value: "Includes",
    label: "does include",
  },
  {
    value: "NotIncludes",
    label: "does not include",
  },
  {
    value: "ExactMatch",
    label: "matches",
  },
  {
    value: "IsNot",
    label: "does not match",
  },
  {
    value: "Less",
    label: "less than",
  },
  {
    value: "IsEqual",
    label: "is equal to",
  },
  {
    value: "Greater",
    label: "greater than",
  },
];

export const searchOpts: TDropdownOpts[] = [
  {
    label: "File Size",
    value: "FileSize",
  },
  {
    label: "File Name",
    value: "FileName",
  },
  {
    label: "Folder Name",
    value: "FolderName",
  },
  {
    label: "File Content",
    value: "FileContent",
  },
  {
    label: "File Extension",
    value: "FileExtension",
  },
  {
    label: "Path Name",
    value: "PathName",
  },
];
