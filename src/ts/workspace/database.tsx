import {useMachine} from "@xstate/react";
import React, {useEffect} from "react";

import SegmentsEmpty from "../../mdx/segments-empty.mdx";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import IntroText from "../common/intro-text";
import Modal from "../common/modal";
import {useAppCtx} from "../context";
import ConfirmDeleteSegment from "../database/confirm-delete-segment";
import DataCard from "../database/data-card";
import SendToVerificationForm from "../forms/send-to-verification";
import {deleteSegment, listSegments, verifySegment} from "../http";
import machine, {DatabaseEventReallyDelete} from "../machines/database";
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

      deleteSegment: async (_ctx, ev) => {
        const {segment} = ev as DatabaseEventReallyDelete;
        await deleteSegment(workspace.slug, segment.slug);
        return segment;
      },
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

    case state.matches("delete_segment"):
    case state.matches("home"):
      return (
        <div>
          <DataCard workspace={workspace} onClick={() => send("SHOW_DATA")} />

          {segments.length > 0 ? (
            <SegmentList
              workspace={workspace}
              segments={segments}
              onExplore={(s) => send("SHOW_SEGMENT", {segment: s})}
              onVerify={(s) => send("SEND_TO_VERIFY", {segment: s})}
              onRemove={(s) => send("DELETE_SEGMENT", {segment: s})}
            />
          ) : (
            <IntroText>
              <SegmentsEmpty />
            </IntroText>
          )}
        </div>
      );

    case state.matches("confirm_delete_segment"): {
      if (state.event.type === "DELETE_SEGMENT") {
        const {segment: eventSegment} = state.event;

        return (
          <div className="fl w-100 pa3">
            <Modal
              onCancel={() => send("SHOW_HOME")}
              title="Delete Segment"
              description="Delete segment."
            >
              <ConfirmDeleteSegment
                segment={eventSegment}
                onCancel={() => send("SHOW_HOME")}
                onDelete={() =>
                  send("REALLY_DELETE_SEGMENT", {
                    segment: eventSegment,
                  })
                }
              />
            </Modal>

            <div>
              <DataCard
                workspace={workspace}
                onClick={() => send("SHOW_DATA")}
              />

              {segments.length > 0 ? (
                <SegmentList
                  workspace={workspace}
                  segments={segments}
                  onExplore={(s) => send("SHOW_SEGMENT", {segment: s})}
                  onVerify={(s) => send("SEND_TO_VERIFY", {segment: s})}
                  onRemove={(s) => send("DELETE_SEGMENT", {segment: s})}
                />
              ) : (
                <IntroText>
                  <SegmentsEmpty />
                </IntroText>
              )}
            </div>
          </div>
        );
      }

      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
    }

    case state.matches("exploration"):
      return (
        <DataTable workspace={workspace} totalStat={total} segment={segment} />
      );

    case state.matches("verify_segment"):
      switch (state.event.type) {
        case "SEND_TO_VERIFY": {
          // I get a Typescript error that segment is not part of the SHOW_HOME
          // event. Although I test for the right event. Not sure how else to solve
          // it besides this weird construct of copying the segment out and assign
          // it to a different variable.
          const {segment: seg} = state.event;
          if (!seg) return <div />;
          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_HOME")}
                title="Send segment to verification."
                description="Describing this modal."
              >
                <div className="flex flex-column">
                  <p>Send a segment to an investigation for verification..</p>

                  <FormHandler
                    onSave={(values) =>
                      verifySegment(workspace.slug, values.investigation, {
                        segment: seg.slug,
                      })
                    }
                    onDone={() => send("SHOW_HOME")}
                    Form={SendToVerificationForm}
                    workspace={workspace}
                  />
                </div>
              </Modal>
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
                    onVerify={(s) => send("SEND_TO_VERIFY", {segment: s})}
                    onRemove={(s) => send("DELETE_SEGMENT", {segment: s})}
                  />
                )}
              </div>
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Database route didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

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
          msg={`Database route didn't match any valid state: ${state.value}`}
          reset={() => appSend("RESTART_APP")}
        />
      );
  }
};

export default Database;
