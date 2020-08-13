import {useMachine} from "@xstate/react";
import React, {useEffect} from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import {useAppCtx} from "../context";
import {listSegments} from "../http";
import machine from "../machines/database";
import {DataStats, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import DataTable from "./data-table";
import SectionCard from "./section-card";
import SegmentList from "./segment-list";

interface DatabaseProps {
  workspace: Workspace;
  stats: DataStats;
  onHeaderChange: (s: string | undefined) => void;
}

const Database = ({workspace, stats, onHeaderChange}: DatabaseProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchSegments: (_ctx, _ev) => listSegments(workspace.slug),
    },

    context: {
      workspace,
      segments: [],
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  const {error, segments} = state.context;
  const segment =
    state.event.type === "SHOW_SEGMENT" ? state.event.segment : undefined;
  const {total} = stats;

  useEffect(() => {
    onHeaderChange(segment ? segment.title : undefined);
  }, [segment, onHeaderChange]);

  switch (true) {
    case state.matches("segments"):
      return <div />;

    case state.matches("home"):
      return (
        <div>
          <SectionCard
            onClick={() => send("SHOW_DATA")}
            kind="data"
            stats={stats}
          />

          {segments.length > 0 && (
            <SegmentList
              workspace={workspace}
              segments={segments}
              onExplore={(s) => send("SHOW_SEGMENT", {segment: s})}
            />
          )}
        </div>
      );

    case state.matches("exploration"):
      return (
        <DataTable workspace={workspace} totalStat={total} segment={segment} />
      );

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
