import {assign, createMachine, DoneInvokeEvent} from "xstate";

export type FormContext<T> = {
  values?: Partial<T>;
  error?: string;
};

export type FormEventSave<T> = {type: "SAVE"; values: T};

export type FormEvent<T> =
  | FormEventSave<T>
  | {type: "CANCEL"}
  | {type: "DONE"}
  | {type: "RETRY"; values: T};

export type FormState<T> =
  | {
      value: "initial";
      context: FormContext<T> & {values: T};
    }
  | {value: "saving"; context: FormContext<T>}
  | {value: "error"; context: FormContext<T> & {error: string}}
  | {value: "done"; context: FormContext<T>};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export default <T extends unknown>(initialValues?: Partial<T>) =>
  createMachine<FormContext<T>, FormEvent<T>, FormState<T>>(
    {
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
              actions: "setValues",
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
              actions: "fail",
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
    },
    {
      actions: {
        setValues: assign({
          values: (_ctx, ev) => {
            const {values} = ev as FormEventSave<T>;
            return values;
          },
        }),

        fail: assign({
          error: (_ctx, ev) => {
            const {data} = ev as DoneInvokeEvent<Error>;
            return data.message;
          },
        }),
      },
    },
  );
