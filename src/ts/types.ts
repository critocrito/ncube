export interface ConfigSettingReq {
  name: string;
  value: string;
}

export interface ConfigSetting {
  name: string;
  value?: string;
  required: boolean;
  restricted: boolean;
  description: string;
}

export type HostConfig = ConfigSetting[];

export interface WorkspaceCommon {
  id: number;
  name: string;
  slug: string;
  description?: string;
  created_at: Date;
  updated_at: Date;
  database: "sqlite" | "http";
  database_path: string;
}

export type WorkspaceLocal = {
  kind: "local";
  location: string;
} & WorkspaceCommon;

export type WorkspaceRemote = {
  kind: "remote";
  location: string;
} & WorkspaceCommon;

export type Workspace = WorkspaceLocal | WorkspaceRemote;

export interface WorkspaceLocalReq {
  kind: "local";
  name: string;
  description?: string;
  database: "sqlite";
}

export interface WorkspaceRemoteReq {
  kind: "remote";
  name: string;
  description?: string;
  database: "http";
  workspace: string;
  endpoint: string;
  account: {
    email: string;
    otp: string;
    password: string;
    password_again: string;
  };
}

export type WorkspaceReq = WorkspaceLocalReq | WorkspaceRemoteReq;

export interface ConnectionDetails {
  name: string;
  workspace: string;
  endpoint: string;
  email: string;
  otp: string;
  created_at: string;
  updated_at: string;
  description?: string;
}
