import "../css/styles.css";

import {MDXProvider} from "@mdx-js/react";
import {useMachine} from "@xstate/react";
import React from "react";
import ReactDOM from "react-dom";
import SwiperCore, {Thumbs} from "swiper/core";

import ExternalLink from "./components/external-link";
import {NcubeProvider} from "./lib/context";
import {useServiceLogger} from "./lib/hooks";
import machine from "./machines/ncube";
import Ncube from "./views/ncube";

// install Swiper's Thumbs component
SwiperCore.use([Thumbs]);

// enable form focus rings when tabbing
// see: https://medium.com/hackernoon/removing-that-ugly-focus-ring-and-keeping-it-too-6c8727fefcd2
const handleFirstTab = (ev: KeyboardEvent) => {
  // the "I am a keyboard user" key
  if (ev.key === "Tab") {
    document.body.classList.add("user-is-tabbing");
    window.removeEventListener("keydown", handleFirstTab);
  }
};

window.addEventListener("keydown", handleFirstTab);

const domContainer = document.querySelector("#app");

const components = {
  a: ExternalLink,
};

const App = () => {
  const [state, send, service] = useMachine(machine);

  useServiceLogger(service, machine.id);

  return (
    <MDXProvider components={components}>
      <NcubeProvider value={[state, send]}>
        <Ncube />
      </NcubeProvider>
    </MDXProvider>
  );
};

ReactDOM.render(<App />, domContainer);
