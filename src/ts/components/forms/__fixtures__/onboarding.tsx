import React, {useState} from "react";

import Onboarding, {OnboardingFormValues} from "../onboarding";

const Wrapper = () => {
  const [state, setState] = useState<OnboardingFormValues | undefined>();

  return (
    <div>
      <Onboarding onSubmit={setState} />
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
