import {
  ConfigSettingReq,
  HostConfig,
  Investigation,
  InvestigationReq,
  Methodology,
  Process,
  ProcessConfigReq,
  ProcessRunReq,
  Segment,
  SegmentReq,
  Source,
  SourceReq,
  SourceTag,
  Unit,
  VerifySegmentReq,
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

type ResponseMapper<T extends unknown, K extends unknown> = (a: T) => K;

type DataResponse = {
  <T>(resp: Response): Promise<T>;
  <T extends unknown, K>(resp: Response, mapper: ResponseMapper<T, K>): Promise<
    K
  >;
};

export const dataResponse: DataResponse = async <T, K>(
  resp: Response,
  mapper?: (a: T) => K,
) => {
  const body: HttpDataResponse<T> = await resp.json();

  switch (body.status) {
    case "success":
      return Promise.resolve(
        mapper === undefined ? body.data : mapper(body.data),
      );

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
export const listSources = async (
  workspace: string,
  pageIndex = 0,
  pageSize = 20,
): Promise<Source[]> => {
  const url = new URL(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources`,
  );

  url.searchParams.append("page", pageIndex.toString());
  url.searchParams.append("size", pageSize.toString());

  const resp = await fetch(url.toString());

  return dataResponse(resp);
};

export const searchSources = async (
  workspace: string,
  query: string,
  pageIndex = 0,
  pageSize = 20,
): Promise<{data: Source[]; total: number}> => {
  const url = new URL(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/sources/search`,
  );

  url.searchParams.append("page", pageIndex.toString());
  url.searchParams.append("size", pageSize.toString());
  url.searchParams.append("q", encodeURIComponent(query));

  const resp = await fetch(url.toString());

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

export const listSourceTags = async (
  workspace: string,
): Promise<SourceTag[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/source-tags`,
  );

  return dataResponse(resp);
};

/*
 * Data
 */
export const listUnits = async (
  workspace: string,
  pageIndex = 0,
  pageSize = 20,
): Promise<Unit[]> => {
  const url = new URL(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/data`,
  );

  url.searchParams.append("page", pageIndex.toString());
  url.searchParams.append("size", pageSize.toString());

  const resp = await fetch(url.toString());

  return dataResponse(resp);
};

export const searchUnits = async (
  workspace: string,
  query: string,
  pageIndex = 0,
  pageSize = 20,
): Promise<{data: Unit[]; total: number}> => {
  const url = new URL(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/data/search`,
  );

  url.searchParams.append("page", pageIndex.toString());
  url.searchParams.append("size", pageSize.toString());
  url.searchParams.append("q", encodeURIComponent(query));

  const resp = await fetch(url.toString());

  return dataResponse(resp);
};

/*
 * Segments
 */
export const listSegments = async (workspace: string): Promise<Segment[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/segments`,
  );

  return dataResponse(resp);
};

export const createSegment = async (
  workspace: string,
  body: SegmentReq,
): Promise<void> => {
  await v.segmentReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/segments`,
    {
      body: JSON.stringify(body),
      method: "POST",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

export const updateSegment = async (
  workspace: string,
  segment: string,
  body: SegmentReq,
): Promise<void> => {
  await v.segmentReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/segments/${segment}`,
    {
      body: JSON.stringify(body),
      method: "PUT",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

/*
 * Processes
 */

export const listProcesses = async (workspace: string): Promise<Process[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/processes`,
  );

  return dataResponse(resp);
};

export const updateProcessConfig = async (
  workspace: string,
  config: ProcessConfigReq,
): Promise<void> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/processes`,
    {
      body: JSON.stringify(config),
      method: "PUT",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

export const runProcess = async (
  workspace: string,
  body: ProcessRunReq,
): Promise<void> => {
  await v.processRunReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/processes`,
    {
      body: JSON.stringify(body),
      method: "POST",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

/*
 * Investigations
 */

export const listInvestigations = async (
  workspace: string,
): Promise<Investigation[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/investigations`,
  );

  return dataResponse(resp);
};

export const listMethodologies = async (
  workspace: string,
): Promise<Methodology[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/methodologies`,
  );

  return dataResponse(resp);
};

export const createInvestigation = async (
  workspace: string,
  body: InvestigationReq,
): Promise<void> => {
  await v.investigationReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/investigations`,
    {
      body: JSON.stringify(body),
      method: "POST",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

export const verifySegment = async (
  workspace: string,
  investigation: string,
  body: VerifySegmentReq,
): Promise<void> => {
  await v.verifySegmentReq.isValid(body);

  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/investigations/${investigation}`,
    {
      body: JSON.stringify(body),
      method: "POST",
      headers: {"Content-Type": "application/json"},
    },
  );

  return emptyResponse(resp);
};

export const listInvestigationSegments = async (
  workspace: string,
  investigation: string,
): Promise<Segment[]> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/investigations/${investigation}/segments`,
  );

  return dataResponse(resp);
};

/*
 * Workspace stats
 */
export const statSourcesTotal = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/sources/total`,
  );

  return dataResponse(resp);
};

export const statSourcesTypes = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/sources/types`,
  );

  return dataResponse(resp);
};

export const statDataTotal = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/data/total`,
  );

  return dataResponse(resp);
};

export const statDataSources = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/data/sources`,
  );

  return dataResponse(resp);
};

export const statDataVideos = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/data/videos`,
  );

  return dataResponse(resp);
};

export const statDataSegments = async (workspace: string): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/data/segments`,
  );

  return dataResponse(resp);
};

export const statProcessesAll = async (
  workspace: string,
  process: string,
): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/processes/${process}/all`,
  );

  return dataResponse(resp);
};

export const statSegmentsUnits = async (
  workspace: string,
  segment: string,
): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/segments/${segment}/units`,
  );

  return dataResponse(resp);
};

export const statInvestigationsTotal = async (
  workspace: string,
): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/investigations/total`,
  );

  return dataResponse(resp);
};

export const statInvestigationsData = async (
  workspace: string,
  investigation: string,
): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/investigations/${investigation}/data`,
  );

  return dataResponse(resp);
};

export const statInvestigationsSegments = async (
  workspace: string,
  investigation: string,
): Promise<number> => {
  const resp = await fetch(
    `http://127.0.0.1:40666/api/workspaces/${workspace}/stats/investigations/${investigation}/segments`,
  );

  return dataResponse(resp);
};
