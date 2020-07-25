import React, {useState} from "react";

import UpdateSegment, {
  UpdateSegmentFormValues,
} from "../../forms/update-segment";

const Wrapper = () => {
  const [state, setState] = useState<UpdateSegmentFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: UpdateSegmentFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => setIsCanceled(true);

  return (
    <div>
      <UpdateSegment
        initialValues={{query: "And I'm a search query", title: "I'm a title."}}
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
