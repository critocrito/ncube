import {FetchMock} from "@react-mock/fetch";
import React, {useState} from "react";

import CreateSource, {CreateSourceFormValues} from "../../forms/create-source";
import {WorkspaceLocal} from "../../types";
import workspace from "../local-workspace.json";
import data from "./source-tags.json";

const resp = {
  status: "success",
  data,
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
      <FetchMock
        matcher="http://127.0.0.1:40666/api/workspaces/my-workspace/source-tags"
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
