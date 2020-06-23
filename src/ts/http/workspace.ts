import {Workspace, WorkspaceReq} from "../types";
import * as v from "../validations";
import {dataResponse, emptyResponse} from ".";

export const list = async (): Promise<Workspace[]> => {
  const resp = await fetch("http://localhost:40666/api/workspaces");

  return dataResponse(resp);
};

export const create = async (body: WorkspaceReq): Promise<void> => {
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

  const resp = await fetch("http://localhost:40666/api/workspaces", {
    body: JSON.stringify(body),
    method: "POST",
    headers: {"Content-Type": "application/json"},
  });

  return emptyResponse(resp);
};

export const show = async (slug: string): Promise<Workspace> => {
  const resp = await fetch(`http://localhost:40666/api/workspaces/${slug}`);

  return dataResponse(resp);
};
