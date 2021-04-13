import React from "react";

import {HttpSuccessResponse} from "../../lib/http";
import {SourceTag, Workspace} from "../../types";
import Discovery from "../../views/discovery";
import {localWorkspace, remoteWorkspace, sourceTags} from "../data";
import {FetchMock} from "../helpers";

const workspacesResp: HttpSuccessResponse<Workspace[]> = {
  status: "success",
  data: [localWorkspace, remoteWorkspace],
};

const sourceTagsResp: HttpSuccessResponse<SourceTag[]> = {
  status: "success",
  data: sourceTags,
};

const Wrapper = () => {
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
              <Discovery />
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
