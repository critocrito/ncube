import React, {useState} from "react";

import CreateWorkspace, {CreateWorkspaceFormValues} from "../create-workspace";

const Wrapper = () => {
  const [state, setState] = useState<CreateWorkspaceFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: CreateWorkspaceFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => setIsCanceled(true);

  return (
    <div>
      <CreateWorkspace onCancel={handleCancel} onSubmit={handleSubmit} />
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
