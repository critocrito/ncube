import {useEffect} from "react";
import {EventObject, Interpreter, StateSchema, Typestate} from "xstate";

export const unreachable = (message?: string): never => {
  if (message === undefined) {
    throw new Error("Unreachable code reached.");
  } else {
    throw new Error(`Unreachable code reached: ${message}`);
  }
};

export const isString = (x: unknown): x is string => {
  return typeof x === "string";
};

export const useServiceLogger = <
  TContext,
  TStateSchema extends StateSchema,
  TTypestate extends Typestate<TContext>,
  TEvent extends EventObject = EventObject
>(
  service: Interpreter<TContext, TStateSchema, TEvent, TTypestate>,
  name?: string,
): void =>
  useEffect(() => {
    if (process.env.NODE_ENV === "production") return;
    let i = 0;

    const subscription = service.subscribe((nextState) => {
      const {
        value,
        context,
        event,
        event: {type},
      } = nextState;

      // eslint-disable-next-line no-console
      if (name) console.groupCollapsed(`${name}: ${type} -> ${value} (${i})`);
      // eslint-disable-next-line no-console
      console.log(context, event);
      // eslint-disable-next-line no-console
      if (name) console.groupEnd();

      i += 1;

      return (): void => {
        subscription.unsubscribe();
      };
    });
  }, [service, name]);
