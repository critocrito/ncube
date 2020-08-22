import {useMachine} from "@xstate/react";
import React, {useEffect} from "react";
import {EventObject} from "xstate";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import Modal from "../common/modal";
import {useAppCtx} from "../context";
import CreateInvestigationForm from "../forms/create-investigation";
import {createInvestigation, listInvestigations} from "../http";
import machine from "../machines/investigation";
import {
  Investigation as InvestigationType,
  Segment,
  SegmentUnit,
  Workspace,
} from "../types";
import {useServiceLogger} from "../utils";
import InvestigationDetails from "./investigation-details";
import InvestigationList from "./investigation-list";
import Verification from "./verification";
import VerificationDetails from "./verification-details";

interface InvestigationProps {
  workspace: Workspace;
  onHeaderChange: (s: string | undefined) => void;
}

const Investigation = <
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
>({
  workspace,
  onHeaderChange,
}: InvestigationProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchInvestigations: (_ctx, _ev) => listInvestigations(workspace.slug),
    },

    context: {
      workspace,
      investigations: [],
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  const {error, investigations} = state.context;

  const investigation =
    state.event.type === "SHOW_DETAILS" ? state.event.investigation : undefined;

  const segment =
    state.event.type === "VERIFY_SEGMENT" ? state.event.segment : undefined;

  useEffect(() => {
    if (investigation !== undefined) {
      onHeaderChange(investigation.title);
    } else if (segment !== undefined) {
      onHeaderChange(segment.title);
    } else {
      onHeaderChange(undefined);
    }
  }, [investigation, segment, onHeaderChange]);

  switch (true) {
    case state.matches("investigations"):
      return <div />;

    case state.matches("home"):
      return (
        <div className="flex flex-column">
          <div className="flex mb3">
            <Button
              className="ml-auto"
              size="large"
              onClick={() => send("CREATE_INVESTIGATION")}
            >
              Create New
            </Button>
          </div>

          <InvestigationList
            onClick={(i: InvestigationType) =>
              send("SHOW_DETAILS", {investigation: i})
            }
            investigations={investigations}
          />
        </div>
      );

    case state.matches("details"): {
      if (state.event.type === "SHOW_DETAILS") {
        const {investigation: i} = state.event;
        return (
          <InvestigationDetails
            workspace={workspace}
            investigation={i}
            onVerify={(s: Segment) =>
              send("VERIFY_SEGMENT", {
                segment: s,
                investigation,
              })
            }
          />
        );
      }
      return (
        <Fatal
          msg={`Investigation details didn't match any valid state: ${state.value}`}
          reset={() => send("RETRY")}
        />
      );
    }

    case state.matches("create"): {
      return (
        <div>
          <Modal
            onCancel={() => send("SHOW_HOME")}
            title="Create Investigation"
            description="Create new investigations."
          >
            <div className="flex flex-column">
              <p>Add a new data source for your workspace.</p>

              <FormHandler
                onSave={(values) => createInvestigation(workspace.slug, values)}
                onDone={() => send("SHOW_HOME")}
                Form={CreateInvestigationForm}
                workspace={workspace}
              />
            </div>
          </Modal>
          <div className="flex flex-column">
            <div className="flex mb3">
              <Button
                className="ml-auto"
                size="large"
                onClick={() => send("CREATE_INVESTIGATION")}
              >
                Create New
              </Button>
            </div>

            <InvestigationList
              onClick={(i: InvestigationType) =>
                send("SHOW_DETAILS", {investigation: i})
              }
              investigations={investigations}
            />
          </div>
        </div>
      );
    }

    case state.matches("segment"):
      switch (state.event.type) {
        case "VERIFY_SEGMENT": {
          const i = state.event.investigation;
          const s = state.event.segment;

          return (
            <Verification
              workspace={workspace}
              segment={state.event.segment}
              investigation={state.event.investigation}
              onDetails={(unit: SegmentUnit<TContext, TEvent>) =>
                send("SHOW_UNIT", {
                  investigation: i,
                  segment: s,
                  unit,
                })
              }
            />
          );
        }

        default:
          return (
            <Fatal
              msg={`Investigation segment didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

    case state.matches("unit"):
      switch (state.event.type) {
        case "SHOW_UNIT": {
          const i = state.event.investigation;
          const s = state.event.segment;
          const u = state.event.unit;

          return (
            <div>
              <Modal
                onCancel={() =>
                  send("VERIFY_SEGMENT", {
                    segment: s,
                    investigation: i,
                  })
                }
                title="Data annotations."
                description="Annotate and verify units of data.."
                className="w-80"
              >
                <VerificationDetails workspace={workspace} unit={u} />
              </Modal>
              <Verification
                workspace={workspace}
                segment={s}
                investigation={i}
                onDetails={(unit: SegmentUnit<TContext, TEvent>) =>
                  send("SHOW_UNIT", {
                    investigation: i,
                    segment: s,
                    unit,
                  })
                }
              />
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Investigation segment didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch investigations."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`Investigation route didn't match any valid state: ${state.value}`}
          reset={() => appSend("RESTART_APP")}
        />
      );
  }
};

export default Investigation;
