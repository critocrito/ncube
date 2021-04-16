import {useActor} from "@xstate/react";
import React from "react";

import SourcesIntroduction from "../../mdx/sources-intro.mdx";
import Error from "../components/error";
import IntroText from "../components/intro-text";
import Modal from "../components/modal";
import Panel from "../components/panel";
import Placeholder from "../components/placeholder";
import SourcesCreate from "../components/sources-create";
import SourcesDelete from "../components/sources-delete";
import SourcesDetails from "../components/sources-details";
import SourcesHome from "../components/sources-home";
import Unreachable from "../components/unreachable";
import {useWorkspaceCtx} from "../lib/context";
import machine, {
  SourceContext,
  SourceMachineInterpreter,
} from "../machines/source";
import {TableMachineInterpreter} from "../machines/table";
import {Source} from "../types";

interface SourcesProps {
  sourcesRef: SourceMachineInterpreter;
}

const Sources = ({sourcesRef}: SourcesProps) => {
  const [state, send] = useActor(sourcesRef);

  if (state.matches("initialize")) return <Placeholder />;

  const {
    workspace,
    sources,
    tableRef,
    total,
  } = state.context as SourceContext & {tableRef: TableMachineInterpreter};

  if (state.matches("sources") || state.matches("home"))
    return (
      <SourcesHome
        sources={sources}
        table={tableRef}
        total={total}
        onShow={(source) => send({type: "SHOW", source})}
        onCreate={() => send({type: "CREATE"})}
        onDelete={(source) => send({type: "DELETE", source})}
      />
    );

  if (state.matches("create")) {
    return (
      <>
        <Modal
          onCancel={() => send({type: "CANCEL"})}
          title="Confirm"
          description="Describing this modal"
        >
          <SourcesCreate
            workspace={workspace}
            onDone={() => send({type: "RELOAD"})}
          />
        </Modal>

        <SourcesHome sources={sources} table={tableRef} total={total} />
      </>
    );
  }

  if (state.matches("details")) {
    const {source} = state.context as SourceContext & {
      source: Source;
    };

    return (
      <>
        <Modal
          onCancel={() => send({type: "HOME"})}
          title="Source Detail"
          description="Describing this modal"
        >
          <SourcesDetails
            onDelete={() => send({type: "DELETE", source})}
            source={source}
          />
        </Modal>

        <SourcesHome sources={sources} table={tableRef} total={total} />
      </>
    );
  }

  if (state.matches("delete")) {
    const {source} = state.context as SourceContext & {source: Source};

    return (
      <>
        <Modal
          onCancel={() => send({type: "HOME"})}
          title="Delete Segment"
          description="Delete segment."
        >
          <SourcesDelete
            workspace={workspace}
            source={source}
            onDone={() => send({type: "HOME"})}
          />
        </Modal>

        <SourcesHome sources={sources} table={tableRef} total={total} />
      </>
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default () => {
  const [
    {
      context: {sourcesRef, header},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel header={header} description="">
      <>
        <IntroText>
          <SourcesIntroduction />
        </IntroText>

        {sourcesRef ? (
          <Sources sourcesRef={sourcesRef} />
        ) : (
          <Error msg="Source actor is not available" recover={() => {}} />
        )}
      </>
    </Panel>
  );
};
