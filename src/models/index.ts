export type TLogLevel = "WARN" | "INFO" | "SUCCESS" | "ERROR";
export type TUnitOpts = "GB" | "KB" | "MB" | "TB" | "B";
export type TSelection = "Any" | "All";
export type TActionOpts = "MOVE" | "COPY" | "DELETE" | "UNLINK" | "RENAME";

export type TitlebarStyle = "macos" | "windows" | "gnome" | "system";

export type TSearch =
  | "FileName"
  | "FileContent"
  | "FileSize"
  | "PathName"
  | "FileExtension"
  | "FolderName";

export type TCondition =
  | "Includes"
  | "NotIncludes"
  | "ExactMatch"
  | "IsNot"
  | "Greater"
  | "IsEqual"
  | "Less";

interface IData {
  text: string[];
  size: [number, TUnitOpts];
}

export interface IRule {
  search: TSearch;
  condition: TCondition;
  data: IData;
}

interface IAction {
  action: TActionOpts;
  path: string;
}

export interface ILog {
  id: string;
  name: string; // listener name
  path: string;
  action: string;
  parentId: string;
  timestamp: string;
  destination: string;
  level: TLogLevel; // log level: INFO | WARN | SUCCESS
  message: string;
}

export interface IListener {
  id: string;
  created: string;
  deep: boolean;
  title: string;
  enabled: boolean;
  monitors: string[];
  selection: TSelection;
  rules: IRule[];
  actions: IAction[];
}

export interface ListenerState {
  listeners: IListener[];
}
