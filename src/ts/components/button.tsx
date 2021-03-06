import c from "clsx";
import React from "react";

interface ButtonProps {
  children?: React.ReactNode;
  className?: string;
  onClick?: () => void;
  kind?: "primary" | "secondary" | "caution";
  size?: "normal" | "large" | "small";
  disabled?: boolean;
  type?: "submit" | "button" | "reset";
  ariaExpanded?: boolean;
  ariaHaspopup?: boolean;
}

const Button = ({
  kind = "primary",
  size = "normal",
  onClick = () => {},
  disabled = false,
  type = "button",
  children,
  className,
  ariaExpanded = false,
  ariaHaspopup = false,
}: ButtonProps) => {
  const classes = c(
    "items-center text-center rounded-sm border border-transparent",
    "text-white font-bold leading-tight no-underline uppercase whitespace-nowrap",
    "px-3 py-2 min-w-min",
    {
      "bg-bittersweet": kind === "primary" && !disabled,
      "bg-sapphire": kind === "secondary" && !disabled,
      "text-sapphire bg-transparent": kind === "caution" && !disabled,
      dim: !disabled,
      "text-gray-dark bg-white border border-gray-dark opacity-50 cursor-not-allowed": disabled,
      "w-48": size === "large",
      "w-28": size === "normal",
      "w-20": size === "small",
    },
    className,
  );

  return (
    <button
      type={type}
      disabled={disabled}
      className={classes}
      onClick={onClick}
      aria-expanded={ariaExpanded}
      aria-haspopup={ariaHaspopup}
    >
      {children}
    </button>
  );
};

export default Button;
