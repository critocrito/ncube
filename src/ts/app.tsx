import {useMachine} from "@xstate/react";
import React from "react";

import BasicPanel from "./common/basic-panel";
import Error from "./common/error";
import Fatal from "./common/fatal";
import FormHandler from "./common/form-handler";
import Modal from "./common/modal";
import {AppProvider} from "./context";
import Dashboard from "./dashboard";
import ConfirmDelete from "./dashboard/confirm-delete";
import LinkWorkspace from "./dashboard/link-workspace";
import CreateWorkspaceForm from "./forms/create-workspace";
import {listWorkspaces, saveWorkspace} from "./handlers";
import {deleteWorkspace, showWorkspace} from "./http";
import machine from "./machines/app";
import Onboarding from "./onboarding";
import {useServiceLogger} from "./utils";
import Workspace from "./workspace";

const App = () => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listWorkspaces: (_ctx, _ev) => listWorkspaces(),

      fetchWorkspace: async (_ctx, {workspace: {slug}}) => {
        const workspace = await showWorkspace(slug);
        return workspace;
      },

      deleteWorkspace: async (_ctx, {workspace, removeLocation}) => {
        await deleteWorkspace(workspace.slug, removeLocation);
        return workspace;
      },
    },
  });

  useServiceLogger(service, machine.id);

  const {workspaces, pubsub} = state.context;

  switch (true) {
    case state.matches("onboarding"):
      return (
        <Onboarding
          onDone={(url: string) => {
            const ws = new WebSocket(url);

            // FIXME: How do I deal with errors?
            ws.addEventListener("open", () => {
              const pipe = pubsub.connect();

              ws.addEventListener("message", ({data}) =>
                pipe(JSON.parse(data)),
              );

              send("SHOW_DASHBOARD", {ws});
            });
          }}
        />
      );

    case state.matches("list_workspaces"):
    case state.matches("show_workspace"):
    case state.matches("delete_workspace"):
    case state.matches("dashboard"):
      return (
        <AppProvider value={[state, send]}>
          <BasicPanel>
            <div className="fl w-100 pa3">
              <Dashboard
                workspaces={workspaces}
                onShow={(workspace) => send("SHOW_WORKSPACE", {workspace})}
                onDelete={(workspace) => send("DELETE_WORKSPACE", {workspace})}
                onLink={() => send("LINK_WORKSPACE")}
                onCreate={() => send("CREATE_WORKSPACE")}
              />
            </div>
          </BasicPanel>
        </AppProvider>
      );

    case state.matches("confirm_delete"): {
      if (state.event.type === "DELETE_WORKSPACE") {
        const {workspace} = state.event;

        return (
          <BasicPanel>
            <div className="fl w-100 pa3">
              <Modal
                onCancel={() => send("SHOW_DASHBOARD")}
                title="Delete Workspace"
                description="Delete workspace."
              >
                <ConfirmDelete
                  workspace={workspace}
                  onCancel={() => send("SHOW_DASHBOARD")}
                  onDelete={(removeLocation) =>
                    send("REALLY_DELETE_WORKSPACE", {
                      workspace,
                      removeLocation,
                    })
                  }
                />
              </Modal>

              <Dashboard
                workspaces={workspaces}
                onShow={(ws) => send("SHOW_WORKSPACE", {workspace: ws})}
                onDelete={(ws) => send("DELETE_WORKSPACE", {workspace: ws})}
                onLink={() => send("LINK_WORKSPACE")}
                onCreate={() => send("CREATE_WORKSPACE")}
              />
            </div>
          </BasicPanel>
        );
      }

      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
    }

    case state.matches("create"):
      return (
        <BasicPanel
          header="Fill in some details about your new workspace."
          description="Before you can create a new local workspace, please fill in some basic configuration."
        >
          <FormHandler
            onSave={saveWorkspace}
            onDone={() => send("RELOAD_WORKSPACES")}
            Form={CreateWorkspaceForm}
          />
        </BasicPanel>
      );

    case state.matches("link"):
      return (
        <BasicPanel
          header="Fill in some details about your new workspace."
          description="Before you can create a new local workspace, please fill in some basic configuration."
        >
          <LinkWorkspace onDone={() => send("RELOAD_WORKSPACES")} />
        </BasicPanel>
      );

    case state.matches("workspace"): {
      if (state.event.type === "SHOW_WORKSPACE") {
        const {workspace} = state.event;

        return (
          <AppProvider value={[state, send]}>
            <Workspace workspaces={workspaces} workspace={workspace} />
          </AppProvider>
        );
      }
      if (state.event.type === "done.invoke.fetchWorkspace") {
        const {data: workspace} = state.event;

        return (
          <AppProvider value={[state, send]}>
            <Workspace workspaces={workspaces} workspace={workspace} />
          </AppProvider>
        );
      }

      return (
        <Fatal
          msg="The workspace state lacks a workspace."
          reset={() => send("RESTART_APP")}
        />
      );
    }

    case state.matches("error"):
      return (
        <Error
          msg={state.context.error || "Failed to fetch workspaces."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
          reset={() => send("RESTART_APP")}
        />
      );
  }
};

export default App;
