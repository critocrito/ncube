import React, {useState} from "react";

import Bootstrap, {BootstrapFormValues} from "../../forms/bootstrap";

const Wrapper = () => {
  const [state, setState] = useState<BootstrapFormValues | undefined>();

  return (
    <div>
      <Bootstrap onSubmit={setState} />
      {state === undefined ? (
        ""
      ) : (
        <div>
          <p>Submitted: {JSON.stringify(state)}</p>
        </div>
      )}
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper />
  </div>
);
