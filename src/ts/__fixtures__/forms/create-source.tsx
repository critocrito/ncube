import React, {useState} from "react";

import CreateSource, {CreateSourceFormValues} from "../../forms/create-source";
import {HttpSuccessResponse} from "../../lib/http";
import {SourceTag, WorkspaceLocal} from "../../types";
import {localWorkspace as workspace, sourceTags} from "../data";
import {FetchMock} from "../helpers";

const resp: HttpSuccessResponse<SourceTag[]> = {
  status: "success",
  data: sourceTags,
};

const Wrapper = () => {
  const [state, setState] = useState<CreateSourceFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: CreateSourceFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => {
    setState(undefined);
    setIsCanceled(true);
  };

  return (
    <div>
      <FetchMock<SourceTag[]>
        matcher="/workspaces/my-workspace/source-tags"
        response={resp}
      >
        <CreateSource
          onSubmit={handleSubmit}
          onCancel={handleCancel}
          workspace={(workspace as unknown) as WorkspaceLocal}
        />
      </FetchMock>
      {state === undefined ? (
        ""
      ) : (
        <div>
          <p>Submitted: {JSON.stringify(state)}</p>
        </div>
      )}

      {isCanceled ? (
        <div>
          <p>Form got canceled.</p>
        </div>
      ) : undefined}
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper />
  </div>
);
