// This index.tsx is used for cosmos-react as an entry point, since our
// default webpack config doesn't produce a single entry point named index,
// but two separate ones, popup and options.
import React from "react";
import ReactDOM from "react-dom";

const App = () => <div />;

const domContainer = document.querySelector("#app");
ReactDOM.render(<App />, domContainer);
