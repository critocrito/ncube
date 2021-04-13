/* eslint @typescript-eslint/explicit-function-return-type: off */
/*
 * See: https://github.com/davidkpiano/xstate/issues/1058#issuecomment-636474629
 *      https://gist.github.com/sw-yx/f18fe6dd4c43fddb3a4971e80114a052
 *      https://github.com/typescript-cheatsheets/react-typescript-cheatsheet/blob/master/README.md#context
 */
import React from "react";
import {EventObject, Interpreter, State} from "xstate";

// import {Sender} from "@xstate/react/types";
import ncubeMachine, {
  NcubeContext,
  NcubeEvent,
  NcubeState,
} from "../machines/ncube";
import workspaceMachine, {
  WorkspaceContext,
  WorkspaceEvent,
  WorkspaceState,
} from "../machines/workspace";

type Sender<TEvent extends EventObject> = (event: TEvent) => void;

type NcubeSchema = typeof ncubeMachine;
type NcubeCtx = [
  State<NcubeContext, NcubeEvent, NcubeSchema, NcubeState>,
  Interpreter<NcubeContext, NcubeSchema, NcubeEvent, NcubeState>["send"],
];

type WorkspaceSchema = typeof workspaceMachine;
type WorkspaceCtx = [
  State<WorkspaceContext, WorkspaceEvent, WorkspaceSchema, WorkspaceState>,
  Sender<WorkspaceEvent>,
];

// This is a trick to allow to create a context provider and hook with an
// undefined initial value, but not having to null check the value inside the
// component. This code was taken from a gist, see the links on top.
const createContext = <A>() => {
  const ctx = React.createContext<A | undefined>(undefined);
  const useCtx = () => {
    const c = React.useContext(ctx);
    if (!c) throw new Error("useCtx must be inside a Provider with a value");
    return c;
  };

  return [useCtx, ctx.Provider] as const;
};

export const [useNcubeCtx, NcubeProvider] = createContext<NcubeCtx>();
export const [
  useWorkspaceCtx,
  WorkspaceProvider,
] = createContext<WorkspaceCtx>();
