import {
  ConfigSettingReq,
  HostConfig,
  Source,
  SourceReq,
  Workspace,
  WorkspaceReq,
} from "./types";
import {unreachable} from "./utils";
import * as v from "./validations";

export interface HttpSuccessResponse<T> {
  status: "success";
  data: T;
}

export interface HttpEmptyResponse {
  status: "empty";
}

export interface HttpErrorResponse {
  status: "error";
  code: number;
  errors: string;
}

export type HttpDataResponse<T> = HttpSuccessResponse<T> | HttpErrorResponse;
export type HttpCreatedResponse = HttpEmptyResponse | HttpErrorResponse;

export const dataResponse = async <T>(resp: Response): Promise<T> => {
  const body: HttpDataResponse<T> = await resp.json();

  switch (body.status) {
    case "success":
      return Promise.resolve(body.data);

    case "error":
      return Promise.reject(body.errors);

    default:
      return unreachable(
        "HTTP response yielded neither a success nor an error.",
      );
  }
};

export const emptyResponse = async (resp: Response): Promise<void> => {
  if (resp.ok) return Promise.resolve();

  const body: HttpErrorResponse = await resp.json();

  switch (body.status) {
    case "error":
      return Promise.reject(body.errors);

    default:
      return unreachable(
        "HTTP response yielded neither a success nor an error.",
      );
  }
};

/*
 * Host Config
 */
export const showConfig = async (): Promise<HostConfig> => {
  const resp = await fetch("http://127.0.0.1:40666/api");

  return dataResponse(resp);
};

export const createConfig = async (body: ConfigSettingReq[]): Promise<void> => {
  await Promise.all(body.map((setting) => v.configSettingReq.isValid(setting)));

  const resp = await fetch("http://127.0.0.1:40666/api", {
    body: JSON.stringify(body),
    method: "POST",
    headers: {"Content-Type": "application/json"},
  });

  return emptyResponse(resp);
};

/*
 * Workspaces
 */
export const listWorkspaces = async (): Promise<Workspace[]> => {
  const resp = await fetch("http://127.0.0.1:40666/api/workspaces");

  return dataResponse(resp);
};

export const createWorkspace = async (body: WorkspaceReq): Promise<void> => {
  // eslint-disable-next-line default-case
  switch (body.kind) {
    case "local": {
      await v.localWorkspaceReq.isValid(body);
      break;
    }
    case "remote": {
      await v.remoteWorkspaceReq.isValid(body);
    }
  }

  const resp = await fetch("http://127.0.0.1:40666/api/workspaces", {
    body: JSON.stringify(body),
    method: "POST",
    headers: {"Content-Type": "application/json"},
  });

  return emptyResponse(resp);
};

export const showWorkspace = async (slug: string): Promise<Workspace> => {
  const resp = await fetch(`http://127.0.0.1:40666/api/workspaces/${slug}`);

  return dataResponse(resp);
};

/*
 * Sources
 */
export const listSources = async (workspace: string): Promise<Source[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources`,
  );

  return dataResponse(resp);
};

export const createSource = async (
  workspace: string,
  body: SourceReq,
): Promise<void> => {
  await v.sourceReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources`,
    {
      body: JSON.stringify(body),
      method: "POST",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

export const removeSource = async (
  workspace: string,
  id: number,
): Promise<void> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources/${id}`,
    {
      method: "DELETE",
    },
  );

  return emptyResponse(resp);
};
