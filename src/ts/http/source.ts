import {Source, SourceReq} from "../types";
import {sourceReq} from "../validations";
import {dataResponse, emptyResponse} from ".";

export const list = async (workspace: string): Promise<Source[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources`,
  );

  return dataResponse(resp);
};

export const create = async (
  workspace: string,
  body: SourceReq,
): Promise<void> => {
  await sourceReq.isValid(body);

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

export const remove = async (workspace: string, id: number): Promise<void> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources/${id}`,
    {
      method: "DELETE",
    },
  );

  return emptyResponse(resp);
};
