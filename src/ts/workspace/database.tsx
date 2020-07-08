import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import {useAppCtx} from "../context";
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
    context: {
      workspace,
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  const {error} = state.context;
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
      return <DataTable workspace={workspace} totalStat={total} />;

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch sources."}
          recover={() => send("RETRY")}
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
