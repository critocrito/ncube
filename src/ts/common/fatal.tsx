import React from "react";

import BasicPanel from "../layout/basic-panel";
import Button from "./button";

interface FatalProps {
  msg: string;
  reset?: () => void;
}

const Fatal = ({msg, reset}: FatalProps) => {
  return (
    <BasicPanel header="Fatal error.">
      <div className="flex flex-column">
        <p>
          Unfortunately something went wrong and this app does not know how to
          proceed. All I got was the following error message:
        </p>

        <p className="error b bg-fair-pink pa4">{msg}</p>

        <p>
          This should not have happened. Unfortunately I don&apos;t know how to
          proceed. The only thing you can do is to restart the application and
          hope for the best.
        </p>
        {reset && (
          <Button onClick={reset} size="large">
            Restart Application
          </Button>
        )}
      </div>
    </BasicPanel>
  );
};

export default Fatal;
