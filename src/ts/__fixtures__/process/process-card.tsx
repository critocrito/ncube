import React from "react";

import ProcessCard from "../../process/process-card";
import {Process} from "../../types";

const process1: Process = {
  id: 1,
  name: "Youtube Video",
  description: "Preserve individual videos from Youtube.",
  config: [
    {
      name: "Youtube API Key",
      key: "youtube",
      kind: "secret",
      description: "Youtube API credentials.",
      template: {api_key: "Youtube API key"},
      value: undefined,
    },
  ],
};

const process2: Process = {
  id: 2,
  name: "Youtube Video",
  description: "Preserve individual videos from Youtube.",
  config: [
    {
      name: "Youtube API Key",
      key: "youtube",
      kind: "secret",
      description: "Youtube API credentials.",
      template: {api_key: "Youtube API key"},
      value: {api_key: "some key"},
    },
  ],
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <ProcessCard process={process1} />
    <ProcessCard process={process2} />
  </div>
);
