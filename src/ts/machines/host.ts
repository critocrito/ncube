import {
  actions,
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  sendParent,
} from "xstate";

import {healthCheck, registerClient, showConfig} from "../lib/http";
import PubSub from "../lib/pubsub";
import {HostConfig} from "../types";

const {log} = actions;

interface HostContext {
  pubsub: PubSub;
  cfg?: HostConfig;
  wsUrl?: string;
  connectionId?: string;
  ws?: WebSocket;
  error?: string;
}

type HostEventConnected = {type: "CONNECTED"; ws: WebSocket};

type HostEvent = {type: "INITIALIZE"} | HostEventConnected;

type HostState =
  | {
      value: "initial" | "healthy";
      context: HostContext;
    }
  | {
      value: "configuration";
      context: HostContext & {cfg: HostConfig};
    }
  | {
      value: "registration";
      context: HostContext & {
        cfg: HostConfig;
        wsUrl: string;
        connectionId: string;
      };
    }
  | {
      value: "connected" | "keepalive";
      context: HostContext & {
        cfg: HostConfig;
        wsUrl: string;
        connectionId: string;
        ws: WebSocket;
      };
    }
  | {
      value: "error";
      context: HostContext & {error: string};
    };

export type HostMachineInterpreter = ActorRefFrom<
  Interpreter<HostContext, HostState, HostEvent>["machine"]
>;

const initialContext: HostContext = {
  pubsub: new PubSub(),
  cfg: undefined,
  wsUrl: undefined,
  connectionId: undefined,
  ws: undefined,
  error: undefined,
};

export default createMachine<HostContext, HostEvent, HostState>(
  {
    id: "host",

    initial: "initial",

    context: initialContext,

    states: {
      initial: {
        invoke: {
          id: "health",
          src: "healthCheck",
          onDone: "healthy",
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      healthy: {
        invoke: {
          id: "config",
          src: "showConfig",
          onDone: {
            target: "configuration",
            actions: "setConfig",
          },
          onError: {
            target: "bootstrap",
          },
        },
      },

      configuration: {
        invoke: {
          id: "register",
          src: "registerClient",
          onDone: {
            target: "registration",
            actions: "registerHost",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      // FIXME: handle error if no connection is established
      registration: {
        invoke: {
          id: "connect",
          src: "connectHost",
        },
        on: {
          CONNECTED: {
            target: "connected",
            actions: "setConnection",
          },
        },
      },

      bootstrap: {
        on: {
          INITIALIZE: "initial",
        },
      },

      connected: {
        entry: "syncHostConfig",

        on: {
          INITIALIZE: "initial",
        },

        after: {
          60000: {
            target: "connected",
            actions: "keepalive",
          },
        },
      },

      keepalive: {
        always: "connected",
      },

      error: {
        on: {
          INITIALIZE: "initial",
        },
      },
    },
  },
  {
    actions: {
      setConfig: assign({
        cfg: (_ctx, ev) => {
          const {data: cfg} = ev as DoneInvokeEvent<HostConfig>;
          return cfg;
        },
      }),

      registerHost: assign((ctx, ev) => {
        const {
          data: {url, uuid},
        } = ev as DoneInvokeEvent<{url: string; uuid: string}>;
        return {
          ...ctx,
          wsUrl: url,
          connectionId: uuid,
        };
      }),

      setConnection: assign({
        ws: (_ctx, ev) => {
          const {ws} = ev as HostEventConnected;
          return ws;
        },
      }),

      syncHostConfig: sendParent(() => ({type: "CONNECTED"})),

      keepalive: log(() => "ensuring the connection to the host", "Keepalive:"),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    services: {
      healthCheck,

      showConfig,

      registerClient,

      connectHost: ({wsUrl, pubsub}) => (cb): void => {
        const ws = new WebSocket(wsUrl as string);

        // FIXME: How do I deal with errors?
        ws.addEventListener("open", () => {
          const pipe = pubsub.connect();

          ws.addEventListener("message", ({data}) => pipe(JSON.parse(data)));

          cb({type: "CONNECTED", ws});
        });
      },
    },
  },
);
