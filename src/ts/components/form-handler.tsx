import {useMachine} from "@xstate/react";
import React, {useMemo} from "react";

import {useServiceLogger} from "../lib/hooks";
import createFormMachine, {FormEventSave} from "../machines/form";
import {FormProps, Workspace} from "../types";
import Placeholder from "./placeholder";
import Unreachable from "./unreachable";

interface FormHandlerProps<T> {
  Form: React.FC<FormProps<T>>;
  initialValues?: Partial<T>;
  onDone: (values?: T) => void;
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
      formDone: () => onDone(),
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
            <span className="text-error">{state.context.error}</span>
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
      return <Placeholder />;

    default:
      return <Unreachable machine={machine.id} state={state.value} />;
  }
};

export default FormHandler;
