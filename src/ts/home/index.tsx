import {useMachine} from "@xstate/react";
import React from "react";

import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import BasicPanel from "../common/basic-panel";
import Button from "../common/button";
import ExternalLink from "../common/external-link";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import Overline from "../common/overline";
import {useAppCtx} from "../context";
import CreateWorkspaceForm from "../forms/create-workspace";
import {saveWorkspace} from "../handlers";
import machine from "../machines/home";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import LinkWorkspace from "./link-workspace";
import WorkspaceListItem from "./workspace-list-item";

interface HomeProps {
  workspaces: Workspace[];
  onDone: () => void;
}

const Home = ({onDone, workspaces}: HomeProps) => {
  const [state, send, service] = useMachine(machine, {
    actions: {
      done: (_ctx) => onDone(),
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  switch (true) {
    case state.matches("home"):
      return (
        <BasicPanel>
          <div>
            <div className="fl w-100 pa3">
              <header className="mb5 mt4">
                <ExternalLink url="https://sugarcubetools.net">
                  <img src={logoIcon} alt="Ncube logo." />
                </ExternalLink>
              </header>
              <Overline label="Workspaces" />
              <ul className="list pl0 mt0 mb0">
                {workspaces.map((workspace) => (
                  <WorkspaceListItem
                    key={`workspace-${workspace.id}`}
                    workspace={workspace}
                    stats={{
                      source: 123,
                      data: 123,
                      process: 42,
                      investigation: 23,
                    }}
                    handleOpen={() =>
                      appSend("SHOW_WORKSPACE", {
                        slug: workspace.slug,
                      })
                    }
                  />
                ))}
              </ul>
            </div>

            <div className="flex justify-between ml-auto w-40 mr2">
              <Button
                kind="secondary"
                size="large"
                onClick={() => send("LINK_WORKSPACE")}
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

    case state.matches("create"):
      return (
        <BasicPanel
          header="Fill in some details about your new workspace."
          description="Before you can create a new local workspace, please fill in some basic configuration."
        >
          <FormHandler
            onSave={saveWorkspace}
            onDone={() => appSend("RELOAD_WORKSPACES")}
            Form={CreateWorkspaceForm}
          />
        </BasicPanel>
      );

    case state.matches("link"):
      return <LinkWorkspace onDone={() => appSend("RELOAD_WORKSPACES")} />;

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default Home;
