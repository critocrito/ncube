import PubSubProvider from "pubsub-js";

import {Notification, NotificationEnvelope} from "./types";

export type Pipe = (envelope: NotificationEnvelope) => void;

/*
 * The `PubSub` class is a thin wrapper around an external Pubsub provider.
 * Producers can connect to publish messages.
 *
 * ```
 * const pubsub = new PubSub();
 *
 * const pipe = pubsub.connect();
 *
 * pipe( ... );
 * ```
 *
 * Consumers of those messages subscribe to a topic to receive any messages they
 * are interested in.
 *
 * ```
 * pubsub.subscribe("task.my-workspace.task-label", (msg) => {});
 * ```
 *
 * The topic is a triplet consisting of the following elements:
 *
 * - message type: There is only one type available at the moment, i.e. `task`.
 * - workspace: The slug of the workspace that runs the task.
 * - label: The name of the task that is being run.
 */
class PubSub {
  pubsub: typeof PubSubProvider;

  taskCache: {[key: string]: Notification[]};

  constructor() {
    this.pubsub = PubSubProvider;
    this.taskCache = {};
  }

  connect(): Pipe {
    const pipe = ({label, workspace, ...data}: NotificationEnvelope): void => {
      const topic = `task.${workspace}.${label}`;
      this.taskCache[topic] = (this.taskCache[topic] || [])
        .concat([data])
        .sort((a, b) => {
          if (a.order > b.order) return 1;
          if (b.order > a.order) return -1;

          return 0;
        });
      this.pubsub.publish(topic, data);
    };

    return pipe;
  }

  subscribe(
    topic: string,
    cb: (msg: Notification, topic: string) => void,
  ): () => void {
    const token = this.pubsub.subscribe(topic, (t: string, msg: Notification) =>
      cb(msg, t),
    );
    const unsubscribe = (): void => this.pubsub.unsubscribe(token);

    return unsubscribe;
  }

  lastMessage(topic: string): Notification | void {
    return (this.taskCache[topic] || []).slice(-1)[0];
  }

  finish(topic: string): void {
    delete this.taskCache[topic];
  }
}

export default PubSub;
