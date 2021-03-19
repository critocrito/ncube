import React, {useState} from "react";

import CreateInvestigation, {
  CreateInvestigationFormValues,
} from "../../forms/create-investigation";
import {WorkspaceLocal} from "../../types";
import {HttpSuccessResponse} from "../../lib/http";
import {
  localWorkspace as workspace,
  methodology1,
  FixtureMethodology,
} from "../data";
import {FetchMock} from "../helpers";

const resp: HttpSuccessResponse<FixtureMethodology[]> = {
  status: "success" as const,
  data: [methodology1],
};

const Wrapper = () => {
  const [state, setState] = useState<
    CreateInvestigationFormValues | undefined
  >();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: CreateInvestigationFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => {
    setState(undefined);
    setIsCanceled(true);
  };

  return (
    <div>
      <FetchMock<FixtureMethodology[]>
        matcher="/workspaces/my-workspace/methodologies"
        response={resp}
      >
        <CreateInvestigation
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
