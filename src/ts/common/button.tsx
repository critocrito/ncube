import c from "classnames";
import React from "react";

interface ButtonProps {
  children?: React.ReactNode;
  className?: string;
  onClick?: () => void;
  kind?: "primary" | "secondary" | "caution";
  size?: "normal" | "large";
  disabled?: boolean;
  type?: "submit" | "button" | "reset";
}

const styles = {
  primary: "btn-bittersweet",
  secondary: "btn-sapphire",
  caution: "btn-black",
};

const sizes = {
  normal: "btn-regular",
  large: "btn-large",
};

const Button = ({
  kind = "primary",
  size = "normal",
  onClick = () => {},
  disabled = false,
  type = "button",
  children,
  className,
}: ButtonProps) => {
  const classes = c(
    styles[kind] !== undefined && !disabled ? styles[kind] : undefined,
    sizes[size] !== undefined ? sizes[size] : undefined,
    disabled ? "btn-disabled" : "btn-active",
    className,
  );

  return (
    <button
      type={type}
      disabled={disabled}
      className={classes}
      onClick={onClick}
    >
      {children}
    </button>
  );
};

export default Button;
