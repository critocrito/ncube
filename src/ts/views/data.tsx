import {useActor} from "@xstate/react";
import React from "react";

import DataIntroduction from "../../mdx/data-intro.mdx";
import HelpText from "../../mdx/search-help.mdx";
import DataDeleteSegment from "../components/data-delete-segment";
import DataDetails from "../components/data-details";
import DataHome from "../components/data-home";
import DataSegmentsCreate from "../components/data-segments-create";
import DataSegmentsHome from "../components/data-segments-home";
import DataSegmentsUpdate from "../components/data-segments-update";
import DataSegmentsVerify from "../components/data-segments-verify";
import Error from "../components/error";
import IntroText from "../components/intro-text";
import Modal from "../components/modal";
import Panel from "../components/panel";
import Placeholder from "../components/placeholder";
import Unreachable from "../components/unreachable";
import {useWorkspaceCtx} from "../lib/context";
import {verifySegment} from "../lib/http";
import machine, {
  DatabaseContext,
  DatabaseMachineInterpreter,
} from "../machines/database";
import {TableMachineInterpreter} from "../machines/table";
import {Segment, Unit} from "../types";

interface DataProps {
  databaseRef: DatabaseMachineInterpreter;
}

const Data = ({databaseRef}: DataProps) => {
  const [state, send] = useActor(databaseRef);

  if (state.matches("initialize")) return <Placeholder />;

  const {
    workspace,
    segments,
    tableRef,
    dataStats,
    total,
  } = state.context as DatabaseContext & {tableRef: TableMachineInterpreter};

  if (state.matches("segments") || state.matches("home"))
    return (
      <DataSegmentsHome
        workspace={workspace}
        segments={segments}
        stats={dataStats}
        onShow={() => send({type: "SHOW_DATA"})}
        onShowSegment={(segment) => send({type: "SHOW_SEGMENT", segment})}
        onVerifySegment={(segment) => send({type: "SEND_TO_VERIFY", segment})}
        onDeleteSegment={(segment) => send({type: "DELETE_SEGMENT", segment})}
      />
    );

  if (state.matches("units")) {
    const {units, segment, searchQuery} = state.context;

    return (
      <DataHome
        units={units}
        query={searchQuery}
        segment={segment}
        table={tableRef}
        total={total}
      />
    );
  }

  if (state.matches("exploration")) {
    const {units, segment, searchQuery} = state.context;

    return (
      <>
        <DataHome
          units={units}
          query={searchQuery}
          segment={segment}
          table={tableRef}
          total={total}
          onShow={(unit) => send({type: "SHOW", unit})}
          onHelp={() => send({type: "SHOW_HELP"})}
          onSearchQuery={(query) => send({type: "SET_QUERY", query})}
          onSearch={() => send({type: "SHOW_DATA"})}
          onCreateSegment={() => send({type: "CREATE_SEGMENT"})}
          onUpdateSegment={() => send({type: "UPDATE_SEGMENT"})}
        />
      </>
    );
  }

  if (state.matches("details")) {
    const {
      units,
      unit,
      segment,
      searchQuery,
    } = state.context as DatabaseContext & {unit: Unit};

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Data Details"
          description="Describing this modal"
        >
          <DataDetails unit={unit} />
        </Modal>

        <DataHome
          units={units}
          query={searchQuery}
          segment={segment}
          table={tableRef}
          total={total}
        />
      </>
    );
  }

  if (state.matches("help")) {
    const {units, segment, searchQuery} = state.context;

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Data Details"
          description="Describing this modal"
        >
          <div className="mdx">
            <HelpText />
          </div>
        </Modal>

        <DataHome
          units={units}
          query={searchQuery}
          segment={segment}
          table={tableRef}
          total={total}
        />
      </>
    );
  }

  if (state.matches("segment_create")) {
    const {units, searchQuery} = state.context;

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_DATA"})}
          title="Confirm"
          description="Please fill in any missing data."
        >
          <DataSegmentsCreate
            initialValues={{query: searchQuery}}
            onDone={() => send({type: "SHOW_DATA"})}
            workspace={workspace}
          />
        </Modal>

        <DataHome
          units={units}
          query={searchQuery}
          table={tableRef}
          total={total}
        />
      </>
    );
  }

  if (state.matches("segment_update")) {
    const {units, segment, searchQuery} = state.context as DatabaseContext & {
      segment: Segment;
    };

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_DATA"})}
          title="Confirm"
          description="Please fill in any missing data."
        >
          <DataSegmentsUpdate
            initialValues={{...segment, query: searchQuery}}
            onDone={() => send({type: "SHOW_DATA"})}
            workspace={workspace}
            segment={segment}
          />
        </Modal>

        <DataHome
          units={units}
          query={searchQuery}
          segment={segment}
          table={tableRef}
          total={total}
        />
      </>
    );
  }

  if (state.matches("segment_delete")) {
    const {segment} = state.context as DatabaseContext & {segment: Segment};

    return (
      <>
        <Modal
          onCancel={() => send({type: "RELOAD"})}
          title="Delete Segment"
          description="Delete segment."
        >
          <DataDeleteSegment
            workspace={workspace}
            segment={segment}
            onDone={() => send({type: "RELOAD"})}
          />
        </Modal>

        <DataSegmentsHome
          workspace={workspace}
          segments={segments}
          stats={dataStats}
        />
      </>
    );
  }

  if (
    state.matches("segment_verify") &&
    state.event.type === "SEND_TO_VERIFY"
  ) {
    const {segment} = state.event;

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Send segment to verification."
          description="Describing this modal."
        >
          <DataSegmentsVerify
            workspace={workspace}
            onCreate={(values) =>
              verifySegment(workspace.slug, values.investigation, {
                segment: segment.slug,
              })
            }
            onDone={() => send({type: "SHOW_HOME"})}
          />
        </Modal>

        <DataSegmentsHome
          workspace={workspace}
          segments={segments}
          stats={dataStats}
        />
      </>
    );
  }

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch sources."}
        recover={() => send({type: "RETRY"})}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default () => {
  const [
    {
      context: {databaseRef, header},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel header={header}>
      <>
        <IntroText>
          <DataIntroduction />
        </IntroText>

        {databaseRef ? (
          <Data databaseRef={databaseRef} />
        ) : (
          <Error msg="Data actor is not available" recover={() => {}} />
        )}
      </>
    </Panel>
  );
};
