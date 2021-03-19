import React from "react";

import ProcessCard from "../../components/processes/card";
import {process1, process2} from "../data";
import {FetchMock, Wrapper} from "../helpers";

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <FetchMock
      matcher="/workspaces/my-workspace/stats/processes/youtube_video/all"
      response={{status: "success", data: 23}}
    >
      <Wrapper>
        <ProcessCard process={process1} onClick={() => {}} onRun={() => {}} />
        <ProcessCard process={process2} onClick={() => {}} onRun={() => {}} />
      </Wrapper>
    </FetchMock>
  </div>
);
