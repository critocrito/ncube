/* eslint no-console: off */
import React, {useState} from "react";

import FileUpload from "../../components/file-upload";

const Wrapper = () => {
  const [state, setState] = useState<Record<string, unknown> | undefined>();

  const handleUpload = (values: Record<string, unknown>) => {
    setState(values);
  };

  return (
    <div>
      <FileUpload handleUpload={handleUpload} handleError={console.log} />
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
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
