import {Workspace, WorkspaceReq} from "../types";
import {dataResponse, emptyResponse} from ".";

export const list = async (): Promise<Workspace[]> => {
  const resp = await fetch("http://localhost:40666/api/workspaces");

  return dataResponse(resp);
};

export const create = async (body: WorkspaceReq): Promise<void> => {
  const resp = await fetch("http://localhost:40666/api/workspaces", {
    body: JSON.stringify(body),
    method: "POST",
    headers: {"Content-Type": "application/json"},
  });

  return emptyResponse(resp);
};
