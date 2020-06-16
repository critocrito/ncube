import c from "classnames";
import React from "react";

interface ButtonProps {
  children?: React.ReactNode;
  className?: string;
  onClick?: () => void;
  type?: "primary" | "secondary" | "caution";
  size?: "normal" | "large";
  disabled?: boolean;
}

const styles = {
  primary: "bg-fresh-frivolous b--fresh-frivolous",
  secondary: "bg-back-to-reality b--back-to-reality",
  caution: "bg-black b--black",
};

const sizes = {
  normal: "small",
  large: "large",
};

const Button = ({
  type = "primary",
  size = "normal",
  onClick = () => {},
  disabled = false,
  children,
  className,
}: ButtonProps) => {
  const classes = c(
    "link white ma2 ttu br2 tc b nowrap",
    styles[type] !== undefined ? styles[type] : undefined,
    sizes[size] !== undefined ? sizes[size] : undefined,
    disabled ? "o-50 bg-white ba b--nasty-color" : "dim pointer",
    className,
  );

  return (
    <button
      type="button"
      disabled={disabled}
      className={classes}
      onClick={onClick}
    >
      {children}
    </button>
  );
};

export default Button;
