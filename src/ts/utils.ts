import {MutableRefObject, Ref, useEffect, useRef, useState} from "react";
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

export const isNumber = (x: unknown): x is number => {
  return typeof x === "number";
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

/**
 * Generate numeric page items around current page.
 *   - Always include first and last page
 *   - Add ellipsis if needed
 */
export const paginate = (
  current: number,
  total: number,
): Array<number | string> => {
  const width = 7;
  if (width % 2 === 0) {
    throw new Error(`Must allow odd number of page items`);
  }
  if (total < width) {
    return [...new Array(total).keys()];
  }
  const left = Math.max(
    0,
    Math.min(total - width, current - Math.floor(width / 2)),
  );
  const items: (string | number)[] = new Array(width);
  for (let i = 0; i < width; i += 1) {
    items[i] = i + left;
  }
  // replace non-ending items with placeholders
  if (items[0] > 0) {
    items[0] = 0;
    items[1] = "…";
  }
  if (items[items.length - 1] < total - 1) {
    items[items.length - 1] = total - 1;
    items[items.length - 2] = "…";
  }
  return items;
};

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
      if (!ref) return;

      if (typeof ref === "function") {
        ref(targetRef.current);
      } else if (ref && ref.current) {
        // eslint-disable-next-line no-param-reassign
        (ref as MutableRefObject<T | null>).current = targetRef?.current;
      }
    });
  }, [refs]);

  return targetRef;
};

export const pick = <T extends Record<string, unknown>, U extends keyof T>(
  keys: U[],
  obj: T,
): Pick<T, U> => {
  return keys.reduce(
    (memo, k) => Object.assign(memo, {[k]: obj[k]}),
    {} as Pick<T, U>,
  );
};
