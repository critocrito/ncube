import test from "ava";

import PubSub from "../../src/ts/pubsub";
import {Notification, NotificationEnvelope} from "../../src/ts/types";

const genMessage = (now: string): NotificationEnvelope => {
  const msg: NotificationEnvelope = {
    kind: "queued",
    task_id: "task_id",
    workspace: "workspace",
    label: "label",
    order: 1,
    created_at: now,
  };
  return msg;
};

test.cb("connect a producer to a subscription", (t) => {
  t.plan(1);

  const now = new Date().toISOString();
  const pubsub = new PubSub();
  const pipe = pubsub.connect();
  pubsub.subscribe("task.workspace.label", (msg: Notification) => {
    t.deepEqual(msg, {
      kind: "queued",
      task_id: "task_id",
      order: 1,
      created_at: now,
    });
    t.end();
  });

  pipe(genMessage(now));
});
