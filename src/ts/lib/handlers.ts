import {CreateSourceFormValues} from "../forms/create-source";
import {CreateWorkspaceFormValues} from "../forms/create-workspace";
import {LinkWorkspaceFormValues} from "../forms/link-workspace";
import {createSource, createWorkspace} from "./http";

export {listWorkspaces} from "./http";

export const saveWorkspace = async (
  data: LinkWorkspaceFormValues | CreateWorkspaceFormValues,
): Promise<void> => {
  if (data.kind === "remote") {
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const {email, otp, password, password_again, ...rest} = data;
    const body = {account: {email, otp, password, password_again}, ...rest};
    return createWorkspace(body);
  }
  if (data.kind === "local") {
    return createWorkspace(data);
  }
  throw new Error(
    "No valid workspace kind detected. Should be either 'remote' or 'local'.",
  );
};

export const saveSource = (
  slug: string,
  values: CreateSourceFormValues,
): Promise<void> => {
  return createSource(slug, values);
};
