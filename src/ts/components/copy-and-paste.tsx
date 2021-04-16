import c from "clsx";
import copy from "copy-to-clipboard";
import React, {useState} from "react";

import icon from "../svg/clipboard.svg";

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

  const copyLabelStyle = c(
    "uppercase text-xs",
    isCopied ? "visible" : "fade-out",
  );

  return (
    <div className="flex items-center">
      <span className={copyLabelStyle}>Copied!</span>
      <button onClick={copyHandler} className="ml-1.5 dim">
        <img src={icon} alt="copy to clipboard." className="h-6 w-6" />
      </button>
    </div>
  );
};

export default CopyAndPaste;
