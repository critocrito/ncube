import {FetchMock} from "@react-mock/fetch";
import React from "react";

import Layout from "../../popup/layout";
import Popup from "../../popup/popup";

const workspacesResp = {
  status: "success",
  data: [
    {
      id: 1,
      name: "Procurement Data",
      slug: "procurement-data",
      description: "",
      created_at: "2020-07-21T19:38:01.203518Z",
      updated_at: "2020-07-21T19:38:01.203518Z",
      kind: "local",
      location: "/Users/crito/Ncube/procurement-data",
      database: "sqlite",
      database_path: "/Users/crito/Ncube/procurement-data/sugarcube.db",
    },
    {
      id: 2,
      name: "the-a-team",
      slug: "the-a-team",
      description: undefined,
      created_at: "2020-07-21T19:52:36.117786Z",
      updated_at: "2020-07-21T19:52:36.117786Z",
      kind: "remote",
      location: "https://ncube.cryptodrunks.net",
      database: "http",
      database_path: "https://ncube.cryptodrunks.net",
    },
  ],
};

const Wrapper = () => {
  const url = "https://www.youtube.com/watch?v=123456";
  const sourceReq = {
    type: "youtube_video",
    term: url,
    tags: [],
  };

  return (
    <FetchMock
      mocks={[
        {
          matcher: "http://127.0.0.1:40666/api/workspaces",
          response: workspacesResp,
        },
        {
          matcher:
            "http://127.0.0.1:40666/api/workspaces/procurement-data/sources",
          response: 201,
          method: "POST",
        },
        {
          matcher: "http://127.0.0.1:40666/api/workspaces/the-a-team/sources",
          response: 201,
          method: "POST",
        },
      ]}
    >
      <Layout>
        <Popup sourceReq={sourceReq} />
      </Layout>
    </FetchMock>
  );
};

export default (
  <div
    style={{width: "600px"}}
    className="noto lh-copy pa2 flex flex-column bg-canvas ba b--black br2"
  >
    <Wrapper />
  </div>
);
