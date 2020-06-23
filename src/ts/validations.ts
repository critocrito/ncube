import * as Yup from "yup";

export const workspaceLabel = Yup.string()
  .required("This workspace requires an identifier.")
  .max(150, "A workspace name cannot have more than 150 characters.");

export const workspaceName = Yup.string()
  .required("Please provide a name for this workspace.")
  .max(150, "A workspace name cannot have more than 150 characters.");

export const workspaceDescription = Yup.string().nullable();

export const workspaceEndpoint = Yup.string()
  .url()
  .required("Please provide the endpoint for this remote workspace.");

export const email = Yup.string()
  .email()
  .required("Your account must be linked to an Email address.");

export const otp = Yup.string().required(
  "Provide your One-Time-Password to link to the remote workspace.",
);

export const password = Yup.string()
  .required("Provide a password to reset the One-Time-Password")
  .min(15, "A password must be at least 15 characters long.")
  .max(250, "A password can be at least 250 characters long.");

export const sourceType = Yup.string();

export const sourceTerm = Yup.string();

export const connectionDetailsUpload = Yup.object({
  workspace: workspaceLabel,
  name: workspaceName,
  description: workspaceDescription,
  endpoint: workspaceEndpoint,
  email,
  otp,
});

export const annotation = Yup.object({
  kind: Yup.string()
    .oneOf(["tag"] as const)
    .defined(),
  term: Yup.string().defined(),
});

export const annotations = Yup.array().of(annotation);

export const configSettingReq = Yup.object({
  name: Yup.string().defined(),
  value: Yup.string().defined(),
});

export const sourceReq = Yup.object({
  type: sourceType.defined(),
  term: sourceTerm.defined(),
  annotations: annotations.defined(),
});

export const localWorkspaceReq = Yup.object({
  kind: Yup.string()
    .oneOf(["local"] as const)
    .defined(),
  name: workspaceName,
  description: workspaceDescription,
  database: Yup.string()
    .oneOf(["sqlite"] as const)
    .defined(),
});

export const remoteWorkspaceReq = Yup.object({
  kind: Yup.string()
    .oneOf(["remote"] as const)
    .defined(),
  name: workspaceName,
  description: workspaceDescription,
  database: Yup.string()
    .oneOf(["http"] as const)
    .defined(),
  workspace: workspaceLabel,
  endpoint: workspaceEndpoint,
  account: Yup.object({
    email,
    otp,
    password,
    password_again: Yup.string().oneOf(
      [Yup.ref("password"), undefined],
      "Passwords must match",
    ),
  }),
});
