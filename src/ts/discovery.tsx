import "../css/styles.css";

import {sourceType} from "@sugarcube/source-types";
import React from "react";
import ReactDOM from "react-dom";

import Layout from "./popup/layout";
import Popup from "./popup/popup";
import {currentUrl} from "./popup/utils";

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
