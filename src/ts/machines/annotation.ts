import {assign, createMachine} from "xstate";

import {Annotation} from "../types";

interface AnnotationContext {
  annotations: Annotation[];
  error?: string;
}

type AnnotationEvent = {type: "SHOW_HOME"};

type AnnotationState =
  | {
      value: "listAnnotations" | "home";
      context: AnnotationContext;
    }
  | {
      value: "error";
      context: AnnotationContext & {error: string};
    };

export default createMachine<
  AnnotationContext,
  AnnotationEvent,
  AnnotationState
>({
  id: "annotation",

  initial: "listAnnotations",

  context: {
    annotations: [],
  },

  states: {
    listAnnotations: {
      invoke: {
        src: "fetchAnnotations",
        onDone: {
          target: "home",
          actions: assign({annotations: (_, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    home: {},

    error: {
      on: {
        SHOW_HOME: "home",
      },
    },
  },
});
