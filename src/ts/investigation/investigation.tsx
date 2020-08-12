import {useMachine} from "@xstate/react";
import React from "react";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import Modal from "../common/modal";
import {useAppCtx} from "../context";
import {listInvestigations} from "../http";
import machine from "../machines/investigation";
import {Investigation as InvestigationType, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import InvestigationList from "./investigation-list";

interface InvestigationProps {
  workspace: Workspace;
}

const Investigation = ({workspace}: InvestigationProps) => {
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

  console.log(investigations);

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

    case state.matches("details"):
      switch (state.event.type) {
        case "SHOW_DETAILS": {
          return <span>Details</span>;
        }

        default:
          return (
            <Fatal
              msg={`Investigation didn't match any valid state: ${state.value}`}
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
            <div>Create</div>
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
