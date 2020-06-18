import {ConfigSettingReq, HostConfig} from "../types";
import {dataResponse, emptyResponse} from ".";

export const show = async (): Promise<HostConfig> => {
  const resp = await fetch("http://localhost:40666/api");

  return dataResponse(resp);
};

export const create = async (body: ConfigSettingReq[]): Promise<void> => {
  const resp = await fetch("http://localhost:40666/api", {
    body: JSON.stringify(body),
    method: "POST",
    headers: {"Content-Type": "application/json"},
  });

  return emptyResponse(resp);
};
