import {useMachine} from "@xstate/react";
import React from "react";

// import Button from "../common/button";
// import Error from "../common/error";
import Fatal from "../common/fatal";
// import FormHandler from "../common/form-handler";
// import Modal from "../common/modal";
import {useAppCtx} from "../context";
// import {createSource, listSources, removeSource} from "../http";
import {listSources} from "../http";
import machine from "../machines/database";
import {DataStats, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import DataTable from "./data-table";
import SectionCard from "./section-card";

interface DatabaseProps {
  workspace: Workspace;
  stats: DataStats;
}

const Database = ({workspace, stats}: DatabaseProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchData: (_ctx, _ev) => listSources(workspace.slug),
    },

    context: {
      workspace,
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();
  const {total} = stats;

  switch (true) {
    case state.matches("list_data"):
      return <div />;

    case state.matches("home"):
      return (
        <div>
          <SectionCard
            onClick={() => send("SHOW_DATA")}
            kind="data"
            stats={stats}
          />
        </div>
      );

    case state.matches("exploration"):
      return (
        <DataTable
          workspace={workspace}
          total={total}
          handleSelected={console.log}
        />
      );

    default:
      return (
        <Fatal
          msg={`Source route didn't match any valid state: ${state.value}`}
          reset={() => appSend("RESTART_APP")}
        />
      );
  }
};

export default Database;
