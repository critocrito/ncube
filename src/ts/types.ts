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
  workspace?: Workspace;
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
  label: string;
  description?: string;
}

export interface Source {
  id: number;
  type: string;
  term: string;
  tags: SourceTag[];
}

// export type Stats = Record<string, number>;
export type Stats<T extends string> = {
  [key in T]: number;
};

// export type DataStats = Stats<"total" | "sources" | "videos">;
export type DataStats = Stats<"total" | "sources">;
export type SourceStats = Stats<"total" | "types">;

export type MediaType = "video" | "image" | "url";

export type Media = {
  id_hash: string;
  type: MediaType;
  term: string;
  data?: Record<string, unknown>;
};

export type Download = {
  id_hash: string;
  type: MediaType;
  term: string;
  md5?: string;
  sha256?: string;
  location?: string;
  data?: Record<string, unknown>;
};

export type Unit = {
  id: number;
  id_hash: string;
  content_hash: string;
  source: string;
  unit_id?: string;
  body?: string;
  href?: string;
  author?: string;
  title?: string;
  description?: string;
  language?: string;
  created_at?: Date;
  fetched_at: Date;
  data?: Record<string, unknown>;
  media: Media[];
  downloads: Download[];
  sources: Source[];
};

export type Segment = {
  id: number;
  slug: string;
  title: string;
  query: string;
  created_at: Date;
  updated_at: Date;
};

/*
 * The request types represent request objects to the HTTP API. They are usually
 * used in the `./http/*` functions. Additionally to types I run validations for
 * the requests. Yup allows to infer types based on the validations.
 *
 * UPDATE: I ran into lots of troubles using the SourceReq type for the web
 * extension. I redefined it as a vanialla Typescript type to make the type work.
 * Seems the infering of types causes some trouble. I might want to avoid it.
 */

export type ConfigSettingReq = Yup.InferType<typeof v.configSettingReq>;
export type SourceReq = {
  type: string;
  term: string;
  tags: SourceTag[];
};

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
