import "../css/styles.css";

import {MDXProvider} from "@mdx-js/react";
import {useMachine} from "@xstate/react";
import React from "react";
import ReactDOM from "react-dom";

import ConfirmDelete from "./components/confirm-delete";
import LinkWorkspace from "./components/dashboard/link-workspace";
import Error from "./components/error";
import ExternalLink from "./components/external-link";
import FormHandler from "./components/form-handler";
import Modal from "./components/modal";
import BasicPanel from "./components/panel-basic";
import Unreachable from "./components/unreachable";
import CreateWorkspaceForm from "./forms/create-workspace";
import {AppProvider} from "./lib/context";
import {listWorkspaces, saveWorkspace} from "./lib/handlers";
import {deleteWorkspace, showWorkspace} from "./lib/http";
import PubSub from "./lib/pubsub";
import {useServiceLogger} from "./lib/utils";
import machine, {
  AppEventReallyDeleteWorkspace,
  AppEventShowWorkspace,
} from "./machines/app";
import Dashboard from "./views/dashboard";
import Onboarding from "./views/onboarding";
import Workspace from "./views/workspace";

// enable form focus rings when tabbing
// see: https://medium.com/hackernoon/removing-that-ugly-focus-ring-and-keeping-it-too-6c8727fefcd2
const handleFirstTab = (ev: KeyboardEvent) => {
  // the "I am a keyboard user" key
  if (ev.key === "Tab") {
    document.body.classList.add("user-is-tabbing");
    window.removeEventListener("keydown", handleFirstTab);
  }
};

window.addEventListener("keydown", handleFirstTab);

const domContainer = document.querySelector("#app");

const components = {
  a: ExternalLink,
};

const App = () => {
  const [state, send, service] = useMachine(machine, {
    context: {
      workspaces: [],
      pubsub: new PubSub(),
    },
    services: {
      listWorkspaces: (_ctx, _ev) => listWorkspaces(),

      fetchWorkspace: async (_ctx, ev) => {
        const {
          workspace: {slug},
        } = ev as AppEventShowWorkspace;
        const workspace = await showWorkspace(slug);
        return workspace;
      },

      deleteWorkspace: async (_ctx, ev) => {
        const {workspace, removeLocation} = ev as AppEventReallyDeleteWorkspace;
        await deleteWorkspace(workspace.slug, removeLocation);
        return workspace;
      },
    },
  });

  useServiceLogger(service, machine.id);

  const {workspaces, pubsub} = state.context;

  if (state.matches("onboarding")) {
    return (
      <AppProvider value={[state, send]}>
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
      </AppProvider>
    );
  }
  if (
    state.matches("list_workspaces") ||
    state.matches("show_workspace") ||
    state.matches("delete_workspace") ||
    state.matches("dashboard")
  ) {
    return (
      <AppProvider value={[state, send]}>
        <Dashboard
          workspaces={workspaces}
          onShow={(workspace) => send("SHOW_WORKSPACE", {workspace})}
          onDelete={(workspace) => send("DELETE_WORKSPACE", {workspace})}
          onLink={() => send("LINK_WORKSPACE")}
          onCreate={() => send("CREATE_WORKSPACE")}
        />
      </AppProvider>
    );
  }
  if (state.matches("confirm_delete")) {
    const {workspace} = state.context;

    return (
      <>
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
      </>
    );
  }
  if (state.matches("create")) {
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
  }
  if (state.matches("link")) {
    return (
      <BasicPanel
        header="Fill in some details about your new workspace."
        description="Before you can create a new local workspace, please fill in some basic configuration."
      >
        <LinkWorkspace onDone={() => send("RELOAD_WORKSPACES")} />
      </BasicPanel>
    );
  }
  if (state.matches("workspace")) {
    const {workspace} = state.context;

    return (
      <AppProvider value={[state, send]}>
        <Workspace workspaces={workspaces} workspace={workspace} />
      </AppProvider>
    );
  }
  if (state.matches("error")) {
    return (
      <Error
        msg={state.context.error || "Failed to fetch workspaces."}
        recover={() => send("RETRY")}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

ReactDOM.render(
  <MDXProvider components={components}>
    <App />
  </MDXProvider>,
  domContainer,
);
