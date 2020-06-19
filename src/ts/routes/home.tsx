import {navigate, RouteComponentProps} from "@reach/router";
import {useMachine} from "@xstate/react";
import React from "react";

import Button from "../components/base/button";
import FileUpload from "../components/base/file-upload";
import Overline from "../components/base/overline";
import CreateWorkspaceForm from "../components/forms/create-workspace";
import LinkWorkspaceForm from "../components/forms/link-workspace";
import WorkspaceListItem from "../components/home/workspace-list-item";
import BasicPanel from "../components/layout/basic-panel";
import {create, list} from "../http/workspace";
import HomeMachine from "../machines/home";
import {isString, unreachable} from "../utils";

const Home = (_: RouteComponentProps) => {
  const [state, send] = useMachine(HomeMachine, {
    services: {
      listWorkspaces: async (_ctx, _ev) => list(),
      saveWorkspace: async (_ctx, {data}) => {
        let body;
        if (data.kind === "remote") {
          // eslint-disable-next-line @typescript-eslint/naming-convention
          const {email, otp, password, password_again, ...rest} = data;
          body = {account: {email, otp, password, password_again}, ...rest};
        } else if (data.kind === "local") {
          body = data;
        }

        return create(body);
      },
    },
  });

  switch (true) {
    case state.matches("home"):
      return (
        <BasicPanel>
          <div>
            <div className="fl w-100 pa2">
              <Overline label="Workspaces" />
              <ul className="list pl0 mt0 mb0">
                {state.context.workspaces.map((workspace) => (
                  <WorkspaceListItem
                    key={`workspace-${workspace.id}`}
                    workspace={workspace}
                    stats={{
                      source: 123,
                      data: 123,
                      process: 42,
                      investigation: 23,
                    }}
                    handleOpen={() => navigate(`/w/${workspace.slug}`)}
                  />
                ))}
              </ul>
            </div>
            <div className="flex justify-between ml-auto w-40 mr2">
              <Button
                kind="secondary"
                size="large"
                onClick={() => send("UPLOAD_CONNECTION_DETAILS")}
              >
                Link Workspace
              </Button>
              <Button size="large" onClick={() => send("CREATE_WORKSPACE")}>
                Create Workspace
              </Button>
            </div>
          </div>
        </BasicPanel>
      );

    case state.matches("createWorkspace"):
      return (
        <BasicPanel>
          <CreateWorkspaceForm
            onSubmit={(data) => {
              send("SAVE", {data});
            }}
            onCancel={() => send("CANCEL")}
          />
        </BasicPanel>
      );

    case state.matches("uploadConnectionDetails"):
      return (
        <BasicPanel>
          <div>
            <FileUpload
              handleUpload={(details) => send("NEXT", {details})}
              handleError={(msg: string) => send("ERROR", {msg})}
            />
            <div className="flex justify-between ml-auto fr mt2">
              <Button
                kind="secondary"
                size="large"
                onClick={() => send("CANCEL")}
              >
                Cancel
              </Button>
            </div>
          </div>
        </BasicPanel>
      );

    case state.matches("linkWorkspace"):
      switch (state.event.type) {
        case "NEXT": {
          const {details} = state.event;
          const initialValues = {
            ...details,
            password: "",
            password_again: "",
            description: isString(details.description)
              ? details.description
              : "",
          };

          return (
            <BasicPanel>
              <LinkWorkspaceForm
                initialValues={{
                  kind: "remote",
                  database: "http",
                  ...initialValues,
                }}
                onSubmit={(data) => {
                  send("SAVE", {data});
                }}
                onCancel={() => send("CANCEL")}
              />
            </BasicPanel>
          );
        }
        default:
          return unreachable(
            "The linkWorkspace state was reached by an unhandled event.",
          );
      }

    case state.matches("listWorkspaces"):
    case state.matches("saveWorkspace"):
      return <p>Loading</p>;
    case state.matches("homeError"):
      setTimeout(() => send("RETRY"), 5 * 1000);
      // FIXME: Handle this state better
      return <p className="error b">Home Error</p>;
    case state.matches("saveError"):
      return <p className="error b">Save Error</p>;
    default:
      return unreachable("Home route didn't match any valid state.");
  }
};

export default Home;
