import {MutableRefObject, Ref, useEffect, useRef, useState} from "react";
import {
  EventObject,
  Interpreter,
  SpawnedActorRef,
  StateSchema,
  Typestate,
} from "xstate";

import {voidFn} from "./utils";

export const useServiceLogger = <
  TContext,
  TStateSchema extends StateSchema,
  TTypestate extends Typestate<TContext>,
  TEvent extends EventObject = EventObject
>(
  service:
    | Interpreter<TContext, TStateSchema, TEvent, TTypestate>
    | SpawnedActorRef<TEvent>,
  name?: string,
): void =>
  useEffect(() => {
    if (process.env.NODE_ENV === "production") return voidFn;
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
    });

    return (): void => {
      subscription.unsubscribe();
    };
  }, [service, name]);

// credit to https://dev.to/gabe_ragland/debouncing-with-react-hooks-jci
export const useDebounce = <T extends unknown>(value: T, delay: number): T => {
  // State and setters for debounced value
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(
    () => {
      // Set debouncedValue to value (passed in) after the specified delay
      const handler = setTimeout(() => {
        setDebouncedValue(value);
      }, delay);

      // Return a cleanup function that will be called every time ...
      // ... useEffect is re-called. useEffect will only be re-called ...
      // ... if value changes (see the inputs array below).
      // This is how we prevent debouncedValue from changing if value is ...
      // ... changed within the delay period. Timeout gets cleared and restarted.
      // To put it in context, if the user is typing within our app's ...
      // ... search box, we don't want the debouncedValue to update until ...
      // ... they've stopped typing for more than 500ms.
      return (): void => {
        clearTimeout(handler);
      };
    },
    // Only re-call effect if value changes
    // You could also add the "delay" var to inputs array if you ...
    // ... need to be able to change that dynamically.
    [value, delay],
  );

  return debouncedValue;
};

// Credit: https://github.com/tannerlinsley/react-table/discussions/1989#discussioncomment-1488
// Also see this: https://github.com/DefinitelyTyped/DefinitelyTyped/issues/31065
export const useCombinedRefs = <T extends HTMLElement>(
  ...refs: Ref<T | null>[]
): MutableRefObject<T | null> => {
  // eslint-disable-next-line unicorn/no-null
  const targetRef = useRef<T | null>(null);

  useEffect(() => {
    refs.forEach((ref) => {
      if (ref) {
        if (typeof ref === "function") {
          ref(targetRef.current);
        } else if (ref && ref.current) {
          // eslint-disable-next-line no-param-reassign
          (ref as MutableRefObject<T | null>).current = targetRef?.current;
        }
      }
    });
  }, [refs]);

  return targetRef;
};
