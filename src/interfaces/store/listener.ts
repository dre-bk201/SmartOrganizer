export type Action = [string, string];

export type SelectionType = "Any" | "All";

export interface Rule {
  searchType: SearchType;
  condition: Condition;
  text: string;
}

export interface Log {
  id: string;
  path: string;
  action: string;
  parent_id: string;
  timestamp: string;
  destination: string;
}

export interface Listener {
  id: string;
  deep: boolean;
  title: string;
  enabled: boolean;
  paths: string[];
  selection: SelectionType;
  rules: Rule[];
  actions: Action[];
  logs: Log[];
}

export type SearchType =
  | "FileName"
  | "FileContent"
  | "FileSize"
  | "PathName"
  | "FileExtension"
  | "FolderName";

type Condition =
  | "Includes"
  | "NotIncludes"
  | "ExactMatch"
  | "IsNot"
  | "Greater"
  | "Less";

export interface ListenerData {
  id: string;
  deep: boolean;
  enabled: boolean;
  paths: string[];
  selection: SelectionType;
  rules: Rule[];
  actions: Action[];
}

export interface State {
  listeners: Array<Listener>;
}
