type TLogLevel = "WARN" | "INFO" | "SUCCESS" | "ERROR";
type TUnitOpts = "GB" | "KB" | "MB" | "TB" | "B";
type TSelection = "Any" | "All";
type TActionOpts = "MOVE" | "COPY" | "DELETE" | "UNLINK" | "RENAME";

type TSearch =
  | "FileName"
  | "FileContent"
  | "FileSize"
  | "PathName"
  | "FileExtension"
  | "FolderName";

type TCondition =
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

interface IRule {
  search: TSearch;
  condition: TCondition;
  data: IData;
}

interface IAction {
  action: TActionOpts;
  path: string;
}

interface ILog {
  id: string;
  name: string; // listener name
  path: string;
  action: string;
  parentId: string;
  destination: string;
  timestamp: string;
  level: TLogLevel; // log level: INFO | WARN | SUCCESS
  message: string;
}

interface IListener {
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

type TDropdownOpts = {
  label: string;
  value: string;
  component?: any;
};

interface ListenerState {
  listeners: IListener[];
}
