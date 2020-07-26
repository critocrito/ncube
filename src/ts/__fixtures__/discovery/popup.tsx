import React from "react";

import Layout from "../../popup/layout";
import Popup from "../../popup/popup";

const Wrapper = () => {
  const url = "https://www.youtube.com/watch?v=123456";
  const sourceReq = {
    type: "youtube_video",
    term: url,
    tags: [],
  };

  return (
    <Layout sourceReq={sourceReq}>
      <Popup sourceReq={sourceReq} />
    </Layout>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas mw6 ba b--black br2">
    <Wrapper />
  </div>
);
