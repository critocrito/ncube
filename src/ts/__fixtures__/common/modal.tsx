import React, {useState} from "react";

import Modal from "../../common/modal";

const Wrapper = () => {
  const [expanded, setExpanded] = useState<boolean>(false);

  const toggleExpanded = () => setExpanded(!expanded);

  return (
    <div>
      <button onClick={toggleExpanded}>Open Modal</button>
      {expanded && (
        <Modal
          title="Modal Test"
          description="Modal description"
          onCancel={toggleExpanded}
        >
          <div>I am a modal</div>
        </Modal>
      )}
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper />
  </div>
);
