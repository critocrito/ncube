import c from "clsx";
import copy from "copy-to-clipboard";
import React, {useState} from "react";

import iconClipboard from "../../../resources/public/images/icon_clipboard.svg";

interface CopyAndPasteProps {
  value: string;
}

const CopyAndPaste = ({value}: CopyAndPasteProps) => {
  const [isCopied, setIsCopied] = useState(false);

  const copyHandler = () => {
    copy(value);
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 3000);
  };

  const copyLabelStyle = c("ttu f7", isCopied ? "visible" : "fade-out");

  return (
    <div className="flex items-center">
      <span className={copyLabelStyle}>Copied!</span>
      <button
        onClick={copyHandler}
        className="bg-transparent ba br2 bw0 ml2 dim"
      >
        <img src={iconClipboard} alt="copy to clipboard." className="h1 w1" />
      </button>
    </div>
  );
};

export default CopyAndPaste;
