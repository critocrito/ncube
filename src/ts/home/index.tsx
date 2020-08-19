import {useMachine} from "@xstate/react";
import React from "react";

import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import WorkspacesEmpty from "../../mdx/workspaces-empty.mdx";
import WorkspacesIntroduction from "../../mdx/workspaces-intro.mdx";
import BasicPanel from "../common/basic-panel";
import Button from "../common/button";
import ExternalLink from "../common/external-link";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import IntroText from "../common/intro-text";
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
                <ExternalLink href="https://sugarcubetools.net">
                  <img src={logoIcon} alt="Ncube logo." />
                </ExternalLink>
              </header>

              <IntroText>
                <WorkspacesIntroduction />
              </IntroText>

              {workspaces.length > 0 ? (
                <>
                  <Overline className="pt4" label="Workspaces" />

                  <ul className="list pl0 mt0 mb0">
                    {workspaces.map((workspace) => (
                      <WorkspaceListItem
                        key={`workspace-${workspace.id}`}
                        workspace={workspace}
                        handleOpen={() =>
                          appSend("SHOW_WORKSPACE", {
                            slug: workspace.slug,
                          })
                        }
                      />
                    ))}
                  </ul>
                </>
              ) : (
                <IntroText>
                  <WorkspacesEmpty />
                </IntroText>
              )}
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
            onDone={() => {
              appSend("RELOAD_WORKSPACES");
              send("SHOW_HOME");
            }}
            Form={CreateWorkspaceForm}
          />
        </BasicPanel>
      );

    case state.matches("link"):
      return (
        <LinkWorkspace
          onDone={() => {
            appSend("RELOAD_WORKSPACES");
            send("SHOW_HOME");
          }}
        />
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default Home;
