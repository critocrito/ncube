import {unreachable} from "../utils";

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
