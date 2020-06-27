import {useMachine} from "@xstate/react";
import React from "react";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import Modal from "../common/modal";
import {useAppCtx} from "../context";
import CreateSourceForm, {CreateSourceFormValues} from "../forms/create-source";
import {create, list, remove} from "../http/source";
import machine from "../machines/source";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import SourcesTable from "./sources-table";

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
      deleteSource: (_ctx, {sourceId}) => remove(workspace.slug, sourceId),
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
          <SourcesTable
            sources={sources}
            onCreate={() => send("CREATE_SOURCE")}
            onDelete={(source) => send("DELETE_SOURCE", {source})}
            handleSelected={console.log}
          />
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

    case state.matches("delete"): {
      if (sources === undefined)
        return (
          <Fatal
            msg="The source/home state lacks sources."
            reset={() => appSend("RESTART_APP")}
          />
        );

      switch (state.event.type) {
        case "DELETE_SOURCE":
          // eslint-disable-next-line no-case-declarations
          const {type, term, id} = state.event.source;

          return (
            <div>
              <Modal
                onCancel={() => send("CANCEL")}
                title="Confirm delete."
                description={`Confirm you want to delete ${term}.`}
              >
                <div className="flex flex-column">
                  <p>Are you sure you want to delete the following source?</p>

                  <dl className="pa4 mt0 sapphire">
                    <dt className="f6 b">Type</dt>
                    <dd className="ml0">{type}</dd>
                    <dt className="f6 b mt2">Term</dt>
                    <dd className="ml0">{term}</dd>
                  </dl>

                  <div className="flex justify-between mt3 ml-auto">
                    <Button
                      className="mr3"
                      type="reset"
                      size="large"
                      kind="secondary"
                      onClick={() => send("CANCEL")}
                    >
                      Cancel
                    </Button>

                    <Button
                      className="mr3 fr"
                      type="submit"
                      size="large"
                      onClick={() => send("CONFIRM_DELETE", {sourceId: id})}
                    >
                      Delete Source
                    </Button>
                  </div>
                </div>
              </Modal>
              <SourcesTable
                sources={sources}
                onCreate={() => send("CREATE_SOURCE")}
                onDelete={(source) => send("DELETE_SOURCE", {source})}
                handleSelected={console.log}
              />
            </div>
          );

        default:
          return (
            <Fatal
              msg={`An unknown event is triggering this state: ${state.event.type}`}
              reset={() => appSend("RESTART_APP")}
            />
          );
      }
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
          msg={`Source route didn't match any valid state: ${state.value}`}
          reset={() => appSend("RESTART_APP")}
        />
      );
  }
};

export default Source;
