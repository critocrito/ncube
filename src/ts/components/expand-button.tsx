import c from "clsx";
import React, {useState} from "react";

import Button from "./button";
import {useOnOutsideClick} from "../lib/hooks";

interface ExpansionItemProps {
  onClick?: () => void;
  children?: React.ReactNode;
  className?: string;
  disabled?: boolean;
}

const ExpansionItem = ({
  onClick,
  children,
  className,
  disabled = false,
}: ExpansionItemProps) => {
  const classes = c(
    "text-sapphire text-left px-4 py-2 bg-white w-full",
    {
      "cursor-not-allowed opacity-40": disabled,
      "hover:bg-canvas": !disabled,
    },
    className,
  );

  return (
    <div className="table-row">
      <div className="border border-sapphire table-cell">
        <button
          onClick={onClick}
          disabled={disabled}
          className={classes}
          role="menuitem"
        >
          {children}
        </button>
      </div>
    </div>
  );
};

interface ExpandButtonProps {
  label: string;
  children?: (Item: typeof ExpansionItem) => React.ReactNode;
  className?: string;
  kind?: "primary" | "secondary" | "caution";
  size?: "normal" | "large";
  disabled?: boolean;
}

const ExpandButton = ({
  label,
  kind = "primary",
  size = "large",
  disabled = false,
  children,
  className,
}: ExpandButtonProps) => {
  const [expanded, setExpanded] = useState(false);
  const ref = useOnOutsideClick<HTMLDivElement>(() => setExpanded(false));

  const onClick = () => {
    setExpanded(!expanded);
  };

  return (
    <div className="relative inline-block text-left">
      <Button
        className={c("flex", className)}
        kind={kind}
        size={size}
        disabled={disabled}
        onClick={onClick}
        ariaExpanded={true}
        ariaHaspopup={true}
      >
        {label}

        <svg
          className="-mr-1 ml-auto h-5 w-5"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
          aria-hidden="true"
        >
          <path
            fillRule="evenodd"
            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
            clipRule="evenodd"
          />
        </svg>
      </Button>

      {expanded && children && (
        <div
          ref={ref}
          className="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none"
          role="menu"
          aria-orientation="vertical"
          aria-labelledby="menu-button"
          tabIndex={-1}
        >
          <div className="table border-collapse w-full" role="none">
            {children(ExpansionItem)}
          </div>
        </div>
      )}
    </div>
  );
};

export default ExpandButton;
