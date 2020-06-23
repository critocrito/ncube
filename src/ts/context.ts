/* eslint @typescript-eslint/explicit-function-return-type: off */
/*
 * See: https://github.com/davidkpiano/xstate/issues/1058#issuecomment-636474629
 *      https://gist.github.com/sw-yx/f18fe6dd4c43fddb3a4971e80114a052
 *      https://github.com/typescript-cheatsheets/react-typescript-cheatsheet/blob/master/README.md#context
 */
import React from "react";
import {Interpreter, State} from "xstate";

import appMachine, {AppContext, AppEvent, AppState} from "./machines/app";
import workspaceMachine, {
  WorkspaceContext,
  WorkspaceEvent,
  WorkspaceState,
} from "./machines/workspace";

type AppSchema = typeof appMachine;
type AppCtx = [
  State<AppContext, AppEvent, AppSchema, AppState>,
  Interpreter<AppContext, AppSchema, AppEvent, AppState>["send"],
];

type WorkspaceSchema = typeof workspaceMachine;
type WorkspaceCtx = [
  State<WorkspaceContext, WorkspaceEvent, WorkspaceSchema, WorkspaceState>,
  Interpreter<
    WorkspaceContext,
    WorkspaceSchema,
    WorkspaceEvent,
    WorkspaceState
  >["send"],
];

// This is a trick to allow to create a context provider and hook with an
// undefined initial value, but not having to null check the value inside the
// component. This code was taken from a gist, see the links on top.
const createContext = <A>() => {
  // eslint-disable-next-line unicorn/no-useless-undefined
  const ctx = React.createContext<A | undefined>(undefined);
  const useCtx = () => {
    const c = React.useContext(ctx);
    if (!c) throw new Error("useCtx must be inside a Provider with a value");
    return c;
  };

  return [useCtx, ctx.Provider] as const;
};

export const [useAppCtx, AppProvider] = createContext<AppCtx>();
export const [useWorkspaceCtx, WorkspaceProvider] = createContext<
  WorkspaceCtx
>();
