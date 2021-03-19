import {useMachine} from "@xstate/react";
import React, {useEffect} from "react";
import {EventObject} from "xstate";

import {CreateInvestigationFormValues} from "../../forms/create-investigation";
import {createInvestigation, listInvestigations} from "../../lib/http";
import {useServiceLogger} from "../../lib/utils";
import machine from "../../machines/investigation";
import {
  Investigation as InvestigationType,
  Segment,
  SegmentUnit,
  Workspace,
} from "../../types";
import Error from "../error";
import Modal from "../modal";
import Placeholder from "../placeholder";
import Unreachable from "../unreachable";
import Verification from "../verification";
import VerificationDetails from "../verification/details";
import InvestigationCreate from "./create";
import InvestigationDetails from "./details";
import InvestigationHome from "./home";

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

  const {investigations} = state.context;

  useEffect(() => {
    if (state.matches("details")) {
      const {investigation} = state.context;

      onHeaderChange(investigation.title);
    } else if (state.matches("segment")) {
      const {segment} = state.context;

      onHeaderChange(segment.title);
    } else {
      onHeaderChange(undefined);
    }
  }, [state, onHeaderChange]);

  if (state.matches("investigations")) {
    return <Placeholder />;
  }
  if (state.matches("home")) {
    return (
      <InvestigationHome
        investigations={investigations}
        onCreate={() => send("CREATE_INVESTIGATION")}
        onShow={(i: InvestigationType) =>
          send("SHOW_DETAILS", {investigation: i})
        }
      />
    );
  }
  if (state.matches("create")) {
    return (
      <>
        <Modal
          onCancel={() => send("SHOW_HOME")}
          title="Create Investigation"
          description="Create new investigations."
        >
          <InvestigationCreate
            workspace={workspace}
            onCreate={(values: CreateInvestigationFormValues) =>
              createInvestigation(workspace.slug, values)
            }
            onDone={() => send("SHOW_HOME")}
          />
        </Modal>

        <InvestigationHome
          investigations={investigations}
          onCreate={() => send("CREATE_INVESTIGATION")}
          onShow={(investigation: InvestigationType) =>
            send("SHOW_DETAILS", {investigation})
          }
        />
      </>
    );
  }
  if (state.matches("details")) {
    const {investigation} = state.context;

    return (
      <InvestigationDetails
        workspace={workspace}
        investigation={investigation}
        onVerify={(segment: Segment) =>
          send("VERIFY_SEGMENT", {
            segment,
            investigation,
          })
        }
      />
    );
  }
  if (state.matches("segment")) {
    const {segment, investigation} = state.context;

    return (
      <Verification
        workspace={workspace}
        segment={segment}
        investigation={investigation}
        onDetails={(unit: SegmentUnit<TContext, TEvent>) =>
          send("SHOW_UNIT", {
            investigation,
            segment,
            unit,
          })
        }
      />
    );
  }
  if (state.matches("unit")) {
    const {investigation, segment, unit} = state.context;

    return (
      <div>
        <Modal
          onCancel={() =>
            send("VERIFY_SEGMENT", {
              segment,
              investigation,
            })
          }
          title="Data annotations."
          description="Annotate and verify units of data.."
          className="w-80"
        >
          <VerificationDetails
            workspace={workspace}
            investigation={investigation}
            unit={unit}
          />
        </Modal>
        <Verification
          workspace={workspace}
          segment={segment}
          investigation={investigation}
          onDetails={() => {}}
        />
      </div>
    );
  }
  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch investigations."}
        recover={() => send("RETRY")}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default Investigation;
