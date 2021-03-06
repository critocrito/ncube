import React from "react";

import Button from "./button";
import PanelBasic from "./panel-basic";

interface FatalProps {
  msg: string;
  reset?: () => void;
}

const Fatal = ({msg, reset}: FatalProps) => {
  return (
    <PanelBasic header="Fatal error.">
      <>
        <p>
          Unfortunately something went wrong and this app does not know how to
          proceed. All I got was the following error message:
        </p>

        <p className="text-error bg-fair-pink p-12">{msg}</p>

        <p>
          This should not have happened. Unfortunately I don&apos;t know how to
          proceed. The only thing you can do is to restart the application and
          hope for the best.
        </p>
        {reset && (
          <Button className="float-right mt-3" onClick={reset} size="large">
            Restart Application
          </Button>
        )}
      </>
    </PanelBasic>
  );
};

export default Fatal;
