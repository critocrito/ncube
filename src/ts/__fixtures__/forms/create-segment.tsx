import React, {useState} from "react";

import CreateSegment, {
  CreateSegmentFormValues,
} from "../../forms/create-segment";

const Wrapper = () => {
  const [state, setState] = useState<CreateSegmentFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: CreateSegmentFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => setIsCanceled(true);

  return (
    <div>
      <CreateSegment
        initialValues={{query: "I'm a search query"}}
        onCancel={handleCancel}
        onSubmit={handleSubmit}
      />

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
