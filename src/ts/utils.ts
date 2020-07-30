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

export const capitalize = (s: string): string =>
  `${s.charAt(0).toUpperCase()}${s.slice(1)}`;

// Adapted from: https://github.com/Coggle/friendly-truncate
export const truncate = (str: string, length: number): string => {
  // truncate string by chopping it in the middle, preferring to cut at word
  // boundaries (whitespace and punctuation), then re-joining with the
  // character join.

  // nothing to do if the length is longer than string:
  if (str.length <= length) return str;
  // special case negative and zero lengths for robustness:
  if (length <= 0) return "";

  const join = "…";
  const boundary = /[\s,.:;_-]/g;
  const tolerance = Math.min(20, Math.round(length / 4));

  const wordBoundariesStart = [];
  const wordBoundariesEnd = [];
  const possibleCut = [];
  let next;
  let firstCut;
  let secondCut;
  let resultLength;

  while (true) {
    next = boundary.exec(str);
    if (!next) break;

    // prune word boundaries to those within length/2 + tolerance of the
    // start/end of the string:
    if (next.index < length / 2 + tolerance) {
      wordBoundariesStart.push(next.index);
    } else if (
      next.index < Math.floor(str.length - (length / 2 + tolerance + 1))
    ) {
      // skip ahead to the end of the string, there's no point testing
      // against all of the middle:
      boundary.lastIndex = Math.floor(
        str.length - (length / 2 + tolerance + 1),
      );
    }

    if (next.index > str.length - (length / 2 + tolerance)) {
      wordBoundariesEnd.push(next.index);
    }
  }

  // reset regex state in case caller re-uses it
  boundary.lastIndex = 0;

  for (let i = wordBoundariesStart.length - 1; i >= 0; i -= 1) {
    // search for a suitable second cut
    firstCut = wordBoundariesStart[i];
    for (let j = wordBoundariesEnd.length - 1; j >= 0; j -= 1) {
      resultLength =
        str.length - (wordBoundariesEnd[j] + 1) + wordBoundariesStart[i] + 1;
      secondCut = undefined;
      if (resultLength <= length && resultLength > length - tolerance) {
        secondCut = wordBoundariesEnd[j] + 1;
      }
      if (secondCut) {
        possibleCut.push({
          length: resultLength,
          first: firstCut,
          second: secondCut,
        });
        // The worst case for performance is where the boundary expression matched
        // at every single character and the tolerance is big. To avoid this being
        // quadratic in the tolerance, break as soon as we have an exact
        // match on length:
        if (resultLength === length) {
          i = -1;
          break;
        }
      }
    }
  }

  // sort preferring overall length and approximately equal length of both
  // parts:
  possibleCut.sort((a, b) => {
    // equalness value, [0, 0.999]:
    const equalnessA =
      1 / (Math.abs(a.first - (str.length - a.second)) + 1.001);
    const equalnessB =
      1 / (Math.abs(b.first - (str.length - b.second)) + 1.001);
    return b.length + equalnessB - (a.length + equalnessA);
  });

  if (possibleCut.length > 0) {
    firstCut = possibleCut[0].first;
    secondCut = possibleCut[0].second;
  } else {
    firstCut = Math.floor(length / 2);
    secondCut = str.length - (length - 1 - firstCut);
  }

  // check if we would cut a surrogate pair in half, if so adjust the cut:
  // (NB: we're assuming string containing only valid surrogate pairs here)
  if (/[\uD800-\uDBFF]/.exec(str[firstCut - 1])) {
    if (secondCut < str.length) {
      firstCut += 1;
      secondCut += 1;
    } else {
      firstCut -= 1;
      secondCut -= 1;
    }
  }
  if (/[\uDC00-\uDFFF]/.exec(str[secondCut])) {
    secondCut += 1;
  }

  const firstPart = str.slice(0, Math.max(0, firstCut));
  const secondPart = str.slice(Math.max(0, secondCut));

  return firstPart + join + secondPart;
};
