import React from "react";

import SegmentList from "../../components/data-segments-table";
import {localWorkspace as workspace, segments} from "../data";
import {FetchMock} from "../helpers";

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <FetchMock
      matcher="/workspaces/my-workspace/stats/segments/mf001a/units"
      response={{status: "success", data: 1}}
    >
      <FetchMock
        matcher="/workspaces/my-workspace/stats/segments/rebel-attacks/units"
        response={{status: "success", data: 2}}
      >
        <SegmentList
          onShow={() => {}}
          onVerify={() => {}}
          onDelete={() => {}}
          segments={segments}
          workspace={workspace}
        />
      </FetchMock>
    </FetchMock>
  </div>
);
