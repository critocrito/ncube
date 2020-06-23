import {CreateWorkspaceFormValues} from "../forms/create-workspace";
import {LinkWorkspaceFormValues} from "../forms/link-workspace";
import {create, list} from "../http/workspace";

export const listWorkspaces = list;

export const saveWorkspace = async (
  data: LinkWorkspaceFormValues | CreateWorkspaceFormValues,
): Promise<void> => {
  if (data.kind === "remote") {
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const {email, otp, password, password_again, ...rest} = data;
    const body = {account: {email, otp, password, password_again}, ...rest};
    return create(body);
  }
  if (data.kind === "local") {
    return create(data);
  }
  throw new Error(
    "No valid workspace kind detected. Should be either 'remote' or 'local'.",
  );
};
