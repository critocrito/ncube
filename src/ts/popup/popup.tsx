import "../../css/styles.css";

import React from "react";
import ReactDOM from "react-dom";

(async () => {
  const domContainer = document.querySelector("#app");
  ReactDOM.render(<p>Hello Popup</p>, domContainer);
})();
