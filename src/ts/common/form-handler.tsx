import {useMachine} from "@xstate/react";
import React, {useMemo} from "react";

import createFormMachine, {FormEventSave} from "../machines/form";
import {FormProps, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import Fatal from "./fatal";

interface FormHandlerProps<T> {
  Form: React.FC<FormProps<T>>;
  initialValues?: Partial<T>;
  onDone: () => void;
  onSave?: (values: T) => Promise<void>;
  workspace?: Workspace;
}

const FormHandler = <T extends unknown>({
  onSave = (_values) => Promise.resolve(),
  onDone,
  Form,
  initialValues,
  workspace,
}: FormHandlerProps<T>) => {
  const machine = useMemo(() => createFormMachine(initialValues), [
    initialValues,
  ]);

  const [state, send, service] = useMachine(machine, {
    actions: {
      formDone: (_ctx) => onDone(),
    },
    services: {
      store: (_ctx, ev) => {
        const {values} = ev as FormEventSave<T>;
        return onSave(values);
      },
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("initial"):
    case state.matches("error"):
    case state.matches("saving"): {
      const isDisabled = state.matches("saving");

      return (
        <>
          {state.context.error && (
            <span className="error">{state.context.error}</span>
          )}
          <Form
            onSubmit={(values: T) => send("SAVE", {values})}
            onCancel={() => send("CANCEL")}
            disabled={isDisabled}
            initialValues={state.context.values}
            workspace={workspace}
          />
        </>
      );
    }

    case state.matches("done"):
      return <div />;

    default:
      return (
        <Fatal
          msg={`Form ${machine.id} reached an unhandled state: ${state.value}`}
        />
      );
  }
};

export default FormHandler;
