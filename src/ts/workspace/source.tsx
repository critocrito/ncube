import {useMachine} from "@xstate/react";
import React from "react";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import {useAppCtx} from "../context";
import CreateSourceForm, {CreateSourceFormValues} from "../forms/create-source";
import {create, list} from "../http/source";
import machine from "../machines/source";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";

interface SourceProps {
  workspace: Workspace;
}

const saveSource = (
  slug: string,
  values: CreateSourceFormValues,
): Promise<void> => {
  // FIXME: remove annotations stub
  return create(slug, Object.assign(values, {annotations: []}));
};

const Source = ({workspace}: SourceProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listSources: (_ctx, _ev) => list(workspace.slug),
    },

    context: {
      workspace,
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  const {sources, error} = state.context;

  switch (true) {
    case state.matches("listing"):
      return <div />;

    case state.matches("home"):
      if (sources === undefined)
        return (
          <Fatal
            msg="The source/home state lacks sources."
            reset={() => appSend("RESTART_APP")}
          />
        );

      return (
        <div>
          <Button onClick={() => send("CREATE_SOURCE")} kind="secondary">
            Add New
          </Button>
          <ul className="list pl0">
            {sources.map(({id, term}) => (
              <li key={id}>{term}</li>
            ))}
          </ul>
        </div>
      );

    case state.matches("create"):
      return (
        <FormHandler
          onSave={(values) => saveSource(workspace.slug, values)}
          onDone={() => send("SHOW_HOME")}
          Form={CreateSourceForm}
        />
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

export default Source;
