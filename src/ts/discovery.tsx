import "../css/styles.css";

import {sourceType} from "@sugarcube/source-types";
import React from "react";
import ReactDOM from "react-dom";

import Layout from "./popup/layout";
import Popup from "./popup/popup";
import {currentUrl, isNcubeRunning} from "./popup/utils";

// enable form focus rings when tabbing
// see: https://medium.com/hackernoon/removing-that-ugly-focus-ring-and-keeping-it-too-6c8727fefcd2
const handleFirstTab = (ev: KeyboardEvent) => {
  // the "I am a keyboard user" key
  if (ev.keyCode === 9) {
    document.body.classList.add("user-is-tabbing");
    window.removeEventListener("keydown", handleFirstTab);
  }
};

window.addEventListener("keydown", handleFirstTab);

(async () => {
  const domContainer = document.querySelector("#app");
  const isRunning = await isNcubeRunning();
  const url = await currentUrl();
  const source = sourceType(url);

  let content;
  switch (true) {
    case !isRunning: {
      content = (
        <div>
          Ncube is not running. Start Ncube to queue &quot;{url}&quot; for
          preservation.
        </div>
      );
      break;
    }

    case source === undefined: {
      content = (
        <div>
          The URL coudn&apos;t be recognized. This probably means that the URL
          is not valid or private.
        </div>
      );
      break;
    }

    default:
      content = (
        <Popup
          sourceReq={{
            type: source as string,
            term: url,
            tags: [],
          }}
        />
      );
  }

  ReactDOM.render(
    <div style={{width: "500px"}}>
      <Layout>{content}</Layout>
    </div>,
    domContainer,
  );
})();
