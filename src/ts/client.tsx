import "../css/styles.css";

import React from "react";
import ReactDOM from "react-dom";

import App from "./app";

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

const domContainer = document.querySelector("#app");

ReactDOM.render(<App />, domContainer);
