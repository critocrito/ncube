import React from "react";

import WebExtension from "../../components/webext";
import Layout from "../../components/webext/layout";
import {Workspace, SourceTag} from "../../types";
import {HttpSuccessResponse} from "../../lib/http";
import {FetchMock} from "../helpers";
import {localWorkspace, remoteWorkspace, sourceTags} from "../data";

const workspacesResp: HttpSuccessResponse<Workspace[]> = {
  status: "success",
  data: [localWorkspace, remoteWorkspace],
};

const sourceTagsResp: HttpSuccessResponse<SourceTag[]> = {
  status: "success",
  data: sourceTags,
};

const Wrapper = () => {
  const url = "https://www.youtube.com/watch?v=123456";
  const sourceReq = {
    type: "youtube_video",
    term: url,
    tags: [],
  };

  return (
    <FetchMock<Workspace[]> matcher="/workspaces" response={workspacesResp}>
      <FetchMock<SourceTag[]>
        matcher="/workspaces/my-workspace/source-tags"
        response={sourceTagsResp}
      >
        <FetchMock<SourceTag[]>
          matcher="/workspaces/the-a-team/source-tags"
          response={sourceTagsResp}
        >
          <FetchMock
            matcher="/workspaces/my-workspace/sources"
            response={201}
            method="POST"
          >
            <FetchMock
              matcher="/workspaces/the-a-team/sources"
              response={201}
              method="POST"
            >
              <Layout>
                <WebExtension sourceReq={sourceReq} />
              </Layout>
            </FetchMock>
          </FetchMock>
        </FetchMock>
      </FetchMock>
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
