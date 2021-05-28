import {
  ActorRefFrom,
  EventObject,
  Interpreter,
  spawn as xstateSpawn,
  StateMachine,
  StateSchema,
} from "xstate";

export const voidFn = (): void => {};

export const identity = <T extends unknown>(x: T): T => x;

export const unreachable = (message?: string): never => {
  const error =
    message === undefined
      ? new Error("Unreachable code reached.")
      : new Error(`Unreachable code reached: ${message}`);
  throw error;
};

export const isString = (x: unknown): x is string => {
  return typeof x === "string";
};

export const isNumber = (x: unknown): x is number => {
  return typeof x === "number";
};

/*
 * This works like the regular `spawn` supplied by xstate. The difference is
 * that it attaches a debug log to the actor.
 */
export const spawn = <
  TContext,
  TStateSchema extends StateSchema,
  TEvent extends EventObject = EventObject
>(
  machine: StateMachine<TContext, TStateSchema, TEvent>,
  name?: string,
): ActorRefFrom<Interpreter<TContext, TStateSchema, TEvent>["machine"]> => {
  const actor = xstateSpawn(machine, name);
  let i = 0;

  // FIXME: I think this actually leaks memory, I never clean up the
  //        subscription. But it might be good enough right now for development.
  if (process.env.NODE_ENV !== "production")
    actor.subscribe((nextState) => {
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

  return actor;
};

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
    return [...Array.from({length: total}).keys()];
  }
  const left = Math.max(
    0,
    Math.min(total - width, current - Math.floor(width / 2)),
  );
  const items: (string | number)[] = Array.from({length: width});
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
  if (/[\uD800-\uDBFF]/.test(str[firstCut - 1])) {
    if (secondCut < str.length) {
      firstCut += 1;
      secondCut += 1;
    } else {
      firstCut -= 1;
      secondCut -= 1;
    }
  }
  if (/[\uDC00-\uDFFF]/.test(str[secondCut])) {
    secondCut += 1;
  }

  const firstPart = str.slice(0, Math.max(0, firstCut));
  const secondPart = str.slice(Math.max(0, secondCut));

  return firstPart + join + secondPart;
};

// Due to the behavior of the --strictFunctionTypes compiler flag added in
// TypeScript v2.6. A function of type (e: CustomEvent) => void is no longer
// considered to be a valid instance of EventListener, which takes an Event
// parameter, not a CustomEvent.
// So one way to fix it is to turn off --strictFunctionTypes. Another way is to
// pass in a function that takes an Event and then narrows to CustomEvent via a
// type guard:
export const isMouseEvent = (event: Event): event is MouseEvent => {
  return "detail" in event;
};

export const downloadAsFile = (
  contentType: string,
  filename: string,
  contents: BlobPart,
): void => {
  const blob = new Blob([contents], {type: contentType});
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;

  const clickHandler = (): void => {
    setTimeout(() => {
      URL.revokeObjectURL(url);
      a.removeEventListener("click", clickHandler);
    }, 150);
  };

  a.addEventListener("click", clickHandler, false);
  a.click();
};
