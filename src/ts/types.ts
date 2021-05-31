import {EventObject, MachineConfig, State, StateSchema} from "xstate";
import * as Yup from "yup";

import * as v from "./lib/validations";

export {EventObject} from "xstate";

export type Resource = {id: number};
export type SlugResource = {slug: string} & Resource;

export type Platform = "youtube" | "twitter" | "http";

export type SourceType =
  | "youtube_video"
  | "youtube_channel"
  | "twitter_tweet"
  | "twitter_user"
  | "http_url";

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

export interface SelectOption {
  value: string;
  label: string;
}

export interface DescriptionOption extends SelectOption {
  description?: string;
}

export interface ConfigSetting {
  name: string;
  value?: string;
  required: boolean;
  restricted: boolean;
  description: string;
}

export type HostConfig = ConfigSetting[];

export type RegisterResponse = {
  url: string;
};

export type WorkspaceCommon = SlugResource & {
  name: string;
  description?: string;
  created_at: Date;
  updated_at: Date;
  is_created: boolean;
  database: "sqlite" | "http";
  database_path: string;
};

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

export interface SourceTag {
  label: string;
  description?: string;
}

export type Source = {
  id: number;
  type: string;
  term: string;
  tags: SourceTag[];
};

export type SearchResults<T extends Resource> = {
  data: T[];
  total: number;
};

// export type Stats = Record<string, number>;
export type Stats<T extends string> = {
  [key in T]: number;
};

// export type DataStats = Stats<"total" | "sources" | "videos">;
const dataStats = ["total", "sources", "segments"] as const;
const sourceStats = ["total", "types"] as const;
export type DataStats = Stats<typeof dataStats[number]>;
export type SourceStats = Stats<typeof sourceStats[number]>;

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

export type Unit = Resource & {
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
  created_at?: string;
  fetched_at: string;
  data?: Record<string, unknown>;
  media: Media[];
  downloads: Download[];
  sources: Source[];
  tags: SourceTag[];
};

export type Segment = SlugResource & {
  title: string;
  query: string;
  // FIXME: dates should be Date not string.
  created_at: string;
  updated_at: string;
};

export type ProcessConfig = {
  name: string;
  key: string;
  description: string;
  kind: "secret";
  template: Record<string, string>;
  value?: Record<string, string>;
};

export type Process = Resource & {
  key: string;
  name: string;
  description: string;
  config: ProcessConfig[];
};

export interface MethodologySchema extends StateSchema {
  states: {
    incoming_data: Record<string, unknown>;
    discarded_data: Record<string, unknown>;
    verified_data: Record<string, unknown>;
    [key: string]: StateSchema;
  };
}

export type Methodology = {
  id: number;
  title: string;
  slug: string;
  description?: string;
  process: MachineConfig<
    Record<string, unknown>,
    MethodologySchema,
    EventObject
  >;
  created_at: string;
  updated_at: string;
};

export type Investigation = SlugResource & {
  title: string;
  description?: string;
  methodology: string;
  created_at: string;
  updated_at: string;
};

export type SegmentUnit = Resource & {
  source: string;
  title?: string;
  videos: number;
  images: number;
  state: State<Record<string, unknown>, EventObject>;
  verification: number;
};

export type Verification = {
  methodology: Methodology;
  columns: string[];
  units: Map<string, SegmentUnit[]>;
};

export type Annotation = {
  key: string;
  name: string;
  value: unknown;
  note?: string;
};

export type AnnotationSchema = {
  key: string;
  name: string;
  description?: string;
  kind: "string" | "text" | "datetime" | "boolean" | "selection";
  required?: boolean;
  selections?: string[];
};

export type NotificationCommon = {
  created_at: string;
  task_id: string;
  order: number;
};

export type NotificationQueued = {
  kind: "queued";
} & NotificationCommon;

export type NotificationStart = {
  kind: "start";
} & NotificationCommon;

export type NotificationProgress = {
  kind: "progress";
  data: {msg: string};
} & NotificationCommon;

export type NotificationDone = {
  kind: "done";
} & NotificationCommon;

export type NotificationError = {
  kind: "error";
  data: {error: string};
} & NotificationCommon;

export type Notification =
  | NotificationQueued
  | NotificationStart
  | NotificationProgress
  | NotificationDone
  | NotificationError;

export type NotificationEnvelope = {
  workspace: string;
  label: string;
} & Notification;

export type ProcessConfigReq = {
  key: string;
  value: Record<string, string>;
};

export type ProcessRunReq = {
  key: string;
  kind: "all" | "new" | "selection";
};

export type FileMetadata = {
  size_in_bytes: number;
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

export type SegmentReq = Yup.InferType<typeof v.segmentReq>;

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

export type InvestigationReq = {
  title: string;
  description?: string;
  methodology: string;
};

export type MethodologyReq<
  TContext extends Record<string, unknown>,
  TStateSchema extends MethodologySchema,
  TEvent extends EventObject
> = {
  title: string;
  description?: string;
  process: MachineConfig<TContext, TStateSchema, TEvent>;
};

export type VerifySegmentReq = {
  segment: string;
};

export type SegmentUnitStateReq<
  TContext extends unknown,
  TEvent extends EventObject
> = {
  state: State<TContext, TEvent>;
};

export type AnnotationReq = {
  key: string;
  value: unknown;
  note?: string;
  name: string;
};
