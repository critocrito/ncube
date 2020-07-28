import React from "react";

import ProcessConfig from "../../process/process-config";
import {Process} from "../../types";

const process: Process = {
  id: 1,
  name: "Youtube Video",
  description: "Preserve individual videos from Youtube.",
  config: [
    {
      name: "Twitter API Keys",
      key: "twitter",
      kind: "secret",
      description: "Twitter API Oauth credentials.",
      template: {
        access_token_key: "OAuth 1.0a Access Token",
        access_token_secret: "OAuth 1.0a Access Secret",
        consumer_key: "OAuth 1.0a Consumer Key",
        consumer_secret: "OAuth 1.0a Consumer Secret",
      },
      value: undefined,
    },
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
    <ProcessConfig onCancel={() => {}} onDone={() => {}} process={process} />
  </div>
);
