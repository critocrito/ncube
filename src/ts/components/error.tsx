import React from "react";

import Button from "./button";
import PanelBasic from "./panel-basic";

interface ErrorProps {
  msg: string;
  recover: () => void;
}

const Error = ({msg, recover}: ErrorProps) => {
  return (
    <PanelBasic header="Something went wrong.">
      <div className="flex flex-column">
        <p>
          The last operation didn&apos;t succeed. All I got was the following
          error message:
        </p>

        <p className="error b bg-fair-pink pa4">{msg}</p>

        <p>
          You can try to recover from the error. If the error keeps on happening
          please open an issue.
        </p>

        <Button onClick={recover} size="large">
          Retry
        </Button>
      </div>
    </PanelBasic>
  );
};

export default Error;
