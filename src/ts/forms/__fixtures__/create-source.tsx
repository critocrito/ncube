import React, {useState} from "react";

import CreateSource, {CreateSourceFormValues} from "../create-source";

const Wrapper = () => {
  const [state, setState] = useState<CreateSourceFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const handleSubmit = (values: CreateSourceFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => {
    // eslint-disable-next-line unicorn/no-useless-undefined
    setState(undefined);
    setIsCanceled(true);
  };

  return (
    <div>
      <CreateSource onSubmit={handleSubmit} onCancel={handleCancel} />

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
