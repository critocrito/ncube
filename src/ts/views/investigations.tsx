import {useActor} from "@xstate/react";
import React from "react";

import InvestigationsIntroduction from "../../mdx/investigations-intro.mdx";
import Error from "../components/error";
import IntroText from "../components/intro-text";
import InvestigationsCreate from "../components/investigations-create";
import InvestigationsDetails from "../components/investigations-details";
import InvestigationsHome from "../components/investigations-home";
import Modal from "../components/modal";
import Panel from "../components/panel";
import Placeholder from "../components/placeholder";
import Unreachable from "../components/unreachable";
import VerificationCanvas from "../components/verification-canvas";
import VerificationDetails from "../components/verification-details";
import {CreateInvestigationFormValues} from "../forms/create-investigation";
import {useWorkspaceCtx} from "../lib/context";
import {createInvestigation} from "../lib/http";
import machine, {
  InvestigationContext,
  InvestigationMachineInterpreter,
} from "../machines/investigation";
import {
  Annotation,
  Investigation,
  Segment,
  SegmentUnit,
  Unit,
  Verification,
} from "../types";

interface InvestigationProps {
  investigationRef: InvestigationMachineInterpreter;
}

const Investigations = ({investigationRef}: InvestigationProps) => {
  const [state, send] = useActor(investigationRef);

  const {workspace, investigations} = state.context;

  if (state.matches("investigations")) return <Placeholder />;

  if (state.matches("home"))
    return (
      <InvestigationsHome
        investigations={investigations}
        onShow={(investigation: Investigation) =>
          send({type: "SHOW_DETAILS", investigation})
        }
        onCreate={() => send({type: "CREATE_INVESTIGATION"})}
      />
    );

  if (state.matches("segments"))
    return <InvestigationsHome investigations={investigations} />;

  if (state.matches("create"))
    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Create Investigation"
          description="Create new investigations."
        >
          <InvestigationsCreate
            workspace={workspace}
            onCreate={(values: CreateInvestigationFormValues) =>
              createInvestigation(workspace.slug, values)
            }
            onDone={() => send({type: "SHOW_HOME"})}
          />
        </Modal>

        <InvestigationsHome investigations={investigations} />
      </>
    );

  if (state.matches("details")) {
    const {investigation, segments} = state.context as InvestigationContext & {
      investigation: Investigation;
    };

    return (
      <InvestigationsDetails
        investigation={investigation}
        segments={segments}
        onVerify={(segment: Segment) => send({type: "VERIFY_SEGMENT", segment})}
      />
    );
  }

  if (state.matches("verification")) {
    const {investigation, segments} = state.context as InvestigationContext & {
      investigation: Investigation;
    };

    return (
      <InvestigationsDetails
        investigation={investigation}
        segments={segments}
      />
    );
  }

  if (state.matches("segment_details") || state.matches("progress")) {
    const {
      segment,
      verification: {methodology, units},
    } = state.context as InvestigationContext & {
      segment: Segment;
      verification: Verification;
    };

    return (
      <VerificationCanvas
        workspace={workspace}
        segment={segment}
        methodology={methodology}
        units={units}
        onDetails={(unit) => send({type: "SHOW_UNIT", unit})}
        onMoveEnd={(unitId, from, to, position) =>
          send({type: "MOVE_UNIT", unitId, from, to, position})
        }
      />
    );
  }

  if (state.matches("unit")) {
    const {
      segment,
      verification: {methodology, units},
    } = state.context as InvestigationContext & {
      segment: Segment;
      verification: Verification;
    };

    return (
      <VerificationCanvas
        workspace={workspace}
        segment={segment}
        methodology={methodology}
        units={units}
      />
    );
  }

  if (
    state.matches("unit_details") ||
    state.matches("annotation") ||
    state.matches("annotations")
  ) {
    const {
      investigation,
      segment,
      verification: {methodology, units},
      unit: segmentUnit,
      unitDetails: {unit, annotations},
    } = state.context as InvestigationContext & {
      investigation: Investigation;
      segment: Segment;
      verification: Verification;
      unit: SegmentUnit;
      unitDetails: {unit: Unit; annotations: Annotation[]};
    };

    if (!investigation || !segment || !unit || !annotations)
      return <Unreachable machine={machine.id} state={state.value} />;

    return (
      <>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Data annotations."
          description="Annotate and verify units of data.."
          className="w-80"
        >
          <VerificationDetails
            unit={unit}
            segmentUnit={segmentUnit}
            annotations={annotations}
            onUpdateAnnotation={(annotation) =>
              send({type: "UPDATE_ANNOTATION", annotation})
            }
          />
        </Modal>

        <VerificationCanvas
          workspace={workspace}
          segment={segment}
          methodology={methodology}
          units={units}
        />
      </>
    );
  }

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch investigations."}
        recover={() => send({type: "RETRY"})}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default () => {
  const [
    {
      context: {investigationRef, header},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel header={header}>
      <>
        <IntroText>
          <InvestigationsIntroduction />
        </IntroText>

        {investigationRef ? (
          <Investigations investigationRef={investigationRef} />
        ) : (
          <Error msg="Processes actor is not available" recover={() => {}} />
        )}
      </>
    </Panel>
  );
};
