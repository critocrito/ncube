import "../css/styles.css";

import React from "react";
import ReactDOM from "react-dom";

import Routes from "./routes";

const domContainer = document.querySelector("#app");

ReactDOM.render(<Routes />, domContainer);
