import "../css/styles.css";

import {sourceType} from "@sugarcube/source-types";
import React from "react";
import ReactDOM from "react-dom";

import Layout from "./popup/layout";
import Popup from "./popup/popup";
import {currentUrl} from "./popup/utils";

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
  const url = await currentUrl();
  const sourceReq = {
    type: sourceType(url) || "http_url",
    term: url,
    tags: [],
  };

  ReactDOM.render(
    <div style={{width: "500px"}}>
      <Layout>
        <Popup sourceReq={sourceReq} />
      </Layout>
    </div>,
    domContainer,
  );
})();
