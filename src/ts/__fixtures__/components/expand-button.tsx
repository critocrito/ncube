/* eslint no-console: off */
import React, {useState} from "react";

import ExpandButton from "../../components/expand-button";

const Wrapper = () => {
  const [state, setState] = useState<string | undefined>();

  const handleClick = (value: string) => {
    setState(value);
  };

  return (
    <div>
      <ExpandButton label="Save Segment">
        {(Item) => {
          return (
            <>
              <Item onClick={() => handleClick("One")}>Update Segment</Item>
              <Item onClick={() => handleClick("Two")}>Create new Segment</Item>
            </>
          );
        }}
      </ExpandButton>
      {state === undefined ? (
        ""
      ) : (
        <div>
          <p>Clicked: {state}</p>
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
