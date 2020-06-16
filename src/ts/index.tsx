import "../css/styles.css";

import React from "react";
import ReactDOM from "react-dom";

import Counter from "./counter";

const App = () => {
  return (
    <div>
      <p>Hello World</p>
      <Counter cnt={2} />
    </div>
  );
};

const domContainer = document.querySelector("#app");

ReactDOM.render(<App />, domContainer);
