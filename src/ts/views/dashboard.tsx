import {useActor} from "@xstate/react";
import React from "react";

import DashboardCreateWorkspace from "../components/dashboard-create-workspace";
import DashboardDeleteWorkspace from "../components/dashboard-delete-workspace";
import DashboardHome from "../components/dashboard-home";
import DashboardLinkConnection from "../components/dashboard-link-connection";
import DashboardLinkWorkspace from "../components/dashboard-link-workspace";
import Error from "../components/error";
import Loading from "../components/loading";
import Modal from "../components/modal";
import BasicPanel from "../components/panel-basic";
import Unreachable from "../components/unreachable";
import machine, {
  DashboardContext,
  DashboardMachineInterpreter,
} from "../machines/dashboard";
import {ConnectionDetails, Workspace} from "../types";

interface DashboardProps {
  dashboardRef: DashboardMachineInterpreter;
}

const Dashboard = ({dashboardRef}: DashboardProps) => {
  const [state, send] = useActor(dashboardRef);

  const {workspaces} = state.context;

  if (state.matches("workspaces")) {
    return (
      <>
        <Loading />
        <BasicPanel>
          <DashboardHome workspaces={workspaces} />
        </BasicPanel>
      </>
    );
  }

  if (
    state.matches("workspace") ||
    state.matches("dashboard") ||
    state.matches("details")
  ) {
    return (
      <BasicPanel>
        <DashboardHome
          workspaces={workspaces}
          onShow={(workspace) => send({type: "SHOW_WORKSPACE", workspace})}
          onDelete={(workspace) => send({type: "DELETE_WORKSPACE", workspace})}
          onLink={() => send({type: "LINK_CONNECTION"})}
          onCreate={() => send({type: "CREATE_WORKSPACE"})}
        />
      </BasicPanel>
    );
  }

  if (state.matches("create")) {
    return (
      <BasicPanel
        header="Fill in some details about your new workspace."
        description="Before you can create a new local workspace, please fill in some basic configuration."
      >
        <DashboardCreateWorkspace onDone={() => send({type: "RELOAD"})} />
      </BasicPanel>
    );
  }

  if (state.matches("connection")) {
    return (
      <BasicPanel
        header="Fill in some details about your new workspace."
        description="Before you can create a new local workspace, please fill in some basic configuration."
      >
        <DashboardLinkConnection
          onSubmit={(details) => send({type: "LINK_WORKSPACE", details})}
          onError={(error) => send({type: "ERROR", error})}
          onCancel={() => send({type: "RELOAD"})}
        />
      </BasicPanel>
    );
  }

  if (state.matches("link")) {
    const {connection} = state.context as DashboardContext & {
      connection: ConnectionDetails;
    };

    return (
      <BasicPanel
        header="Fill in some details about your new workspace."
        description="Before you can create a new local workspace, please fill in some basic configuration."
      >
        <DashboardLinkWorkspace
          connection={connection}
          onDone={() => send({type: "RELOAD"})}
        />
      </BasicPanel>
    );
  }

  if (state.matches("delete")) {
    const {workspace} = state.context as DashboardContext & {
      workspace: Workspace;
    };

    return (
      <BasicPanel>
        <>
          <Modal
            onCancel={() => send({type: "RELOAD"})}
            title="Delete Workspace"
            description="Delete workspace."
          >
            <DashboardDeleteWorkspace
              workspace={workspace}
              onDone={() => send({type: "RELOAD"})}
            />
          </Modal>

          <DashboardHome workspaces={workspaces} />
        </>
      </BasicPanel>
    );
  }

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch workspaces."}
        recover={() => send({type: "RETRY"})}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default Dashboard;
