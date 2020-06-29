import * as Yup from "yup";

import * as v from "./validations";

export interface FormValues<T> {
  values: T;
  error: string;
}

export interface FormProps<T> {
  onSubmit: (values: T) => void;
  onCancel?: () => void;
  initialValues?: Partial<T>;
  disabled?: boolean;
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

export interface Annotation {
  kind: "tag";
  term: string;
}

export interface SourceTag {
  name: string;
  value: string;
}

export interface Source {
  id: number;
  type: string;
  term: string;
  tags: SourceTag[];
}

export type Stats = Record<string, number>;
/*
 * The request types represent request objects to the HTTP API. They are
 * usually used in the `./http/*` functions. Additionally to types I run
 * validations for the requests. Yup allows to infer types based on the
 * validations.
 */

export type ConfigSettingReq = Yup.InferType<typeof v.configSettingReq>;
export type SourceReq = Yup.InferType<typeof v.sourceReq>;

/*
 * It is currently tricky (not possible?) to express discriminated union types
 * with Yup. See: https://github.com/jquense/yup/issues/593
 * So I have to keep the types and the validations in sync for the time being.
 */
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

// export type WorkspaceLocalReq = Yup.InferType<typeof v.localWorkspaceReq>;
// export type WorkspaceRemoteReq = Yup.InferType<typeof v.remoteWorkspaceReq>;

export type WorkspaceReq = WorkspaceLocalReq | WorkspaceRemoteReq;
