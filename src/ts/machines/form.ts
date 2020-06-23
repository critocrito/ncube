import {assign, createMachine} from "xstate";

export type FormContext<T> = {
  values?: Partial<T>;
  error?: string;
};
export type FormEvent<T> =
  | {type: "SAVE"; error: undefined; values: T}
  | {type: "CANCEL"}
  | {type: "DONE"}
  | {type: "RETRY"; values: T};

export type FormState<T> =
  | {
      value: "initial";
      context: FormContext<T> & {values: T};
    }
  | {value: "saving"; context: FormContext<T> & {error: undefined}}
  | {value: "error"; context: FormContext<T> & {error: string}}
  | {value: "done"; context: FormContext<T>};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export default <T extends unknown>(initialValues?: Partial<T>) =>
  createMachine<FormContext<T>, FormEvent<T>, FormState<T>>({
    id: "form",

    context: {
      values: initialValues,
      error: undefined,
    },

    initial: "initial",

    states: {
      initial: {
        on: {
          SAVE: {
            target: "saving",
            actions: assign({
              values: (_ctx, ev) => ev.values,
              // eslint-disable-next-line unicorn/no-useless-undefined
              error: (_ctx, _ev) => undefined,
            }),
          },
          CANCEL: "done",
        },
      },

      saving: {
        invoke: {
          src: "store",
          onDone: {
            target: "done",
          },
          onError: {
            target: "error",
            actions: assign({error: (_, {data}) => data.message}),
          },
        },
      },

      error: {
        on: {
          RETRY: "initial",
        },
      },

      done: {
        entry: "formDone",
        type: "final",
      },
    },
  });
