import copy from "copy-to-clipboard";
import React from "react";

import iconClipboard from "../../../resources/public/images/icon_clipboard.svg";

interface CopyAndPasteProps {
  value: string;
}

const CopyAndPaste = ({value}: CopyAndPasteProps) => {
  return (
    <button
      className="b--none bg-transparent"
      onClick={() => {
        copy(value);
      }}
    >
      <img src={iconClipboard} alt="copy to clipboard." className="h1 w1" />
    </button>
  );
};

export default CopyAndPaste;
