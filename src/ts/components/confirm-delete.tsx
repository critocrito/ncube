import {useMachine} from "@xstate/react";
import React from "react";

import machine, {DeleteContext, DeleteEvent} from "../machines/delete";
import Error from "./error";
import Placeholder from "./placeholder";
import Unreachable from "./unreachable";

interface ConfirmDeleteProps<T extends Record<string, unknown>> {
  onDelete: (ctx: DeleteContext, ev: DeleteEvent<T>) => Promise<void>;
  onDone: () => void;
  children: (args: {
    onSubmit: (data: T) => void;
    onCancel: () => void;
  }) => React.ReactNode;
}

const ConfirmDelete = <T extends Record<string, unknown>>({
  onDelete,
  onDone,
  children,
}: ConfirmDeleteProps<T>) => {
  const m = machine<T>();
  const [state, send, service] = useMachine<DeleteContext, DeleteEvent<T>>(m, {
    services: {delete: onDelete},
  });

  service.onDone(onDone);

  const onSubmit = (data: T) => send({type: "YES", data});
  const onCancel = () => send({type: "NO"});

  if (state.matches("confirm")) return <>{children({onSubmit, onCancel})}</>;

  if (
    state.matches("delete") ||
    state.matches("abort") ||
    state.matches("success")
  )
    return <Placeholder />;

  if (state.matches("error")) {
    return (
      <Error
        msg={state.context.error || "Failed to delete resource."}
        recover={() => send({type: "RETRY"})}
      />
    );
  }

  return <Unreachable machine={m.id} state={state.value} />;
};

export default ConfirmDelete;
