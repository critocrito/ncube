import {FetchMock} from "@react-mock/fetch";
import React, {useState} from "react";

import CreateInvestigation, {
  CreateInvestigationFormValues,
} from "../../forms/create-investigation";
import {WorkspaceLocal} from "../../types";
import workspace from "../local-workspace.json";

const resp = {
  status: "success",
  data: [
    {
      id: 1,
      title: "Tutorial",
      slug: "tutorial",
      description: "The methodology used in the Ncube tutorial.",
      process:
        '{"id":"tutorial","initial":"incoming_data","states":{"incoming_data":{"on":{"TO_DESK_RESEARCH":"desk_research","TO_DISCARDED_DATA":"discarded_data"}},"discarded_data":{"on":{"TO_INCOMING_DATA":"incoming_data"}},"verified_data":{"on":{"TO_INCOMING_DATA":"incoming_data","TO_DISCARDED_DATA":"discarded_data"}},"desk_research":{"on":{"TO_SIGN_OFF":"sign_off","TO_DISCARDED_DATA":"discarded_data"},"meta":{"annotations":[{"name":"location","description":"Longer description","kind":"string"},{"name":"narrative","description":null,"kind":"text","required":true}]}},"sign_off":{"on":{"TO_VERIFIED_DATA":"verified_data","TO_DESK_RESEARCH":"desk_research","TO_DISCARDED_DATA":"discarded_data"}}}}',
      created_at: "2020-08-13T07:54:40.659291Z",
      updated_at: "2020-08-13T07:54:40.659291Z",
    },
  ],
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
      <FetchMock
        matcher="http://127.0.0.1:40666/api/workspaces/my-workspace/methodologies"
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
