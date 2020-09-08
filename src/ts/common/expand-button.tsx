import c from "classnames";
import React, {useState} from "react";

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
  return (
    <li className="bt bb b--fair-pink pt2 pb2 pl1 pr1">
      {onClick ? (
        <button
          className={c(
            "b--transparent bg-white pa2 w-100",
            disabled ? "o-40" : "pointer",
          )}
          onClick={onClick}
          disabled={disabled}
        >
          <div className="fl b sapphire">
            <div className={c(className)}> {children}</div>
          </div>
        </button>
      ) : (
        {children}
      )}
    </li>
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

const styles = {
  primary: "btn-bittersweet",
  secondary: "btn-sapphire",
  caution: "btn-link",
};

const sizes = {
  normal: "btn-regular",
  large: "btn-large",
};

const ExpandButton = ({
  label,
  kind = "primary",
  size = "large",
  disabled = false,
  children,
  className,
}: ExpandButtonProps) => {
  const [expanded, setExpanded] = useState(false);

  const classes = c(
    "fr",
    styles[kind] !== undefined && !disabled ? styles[kind] : undefined,
    sizes[size] !== undefined ? sizes[size] : undefined,
    disabled ? "btn-disabled" : "btn-active",
    className,
  );

  const onClick = () => {
    setExpanded(!expanded);
  };

  return (
    <div className="flex flex-column">
      <div className="w5">
        <button disabled={disabled} className={classes} onClick={onClick}>
          {label}
        </button>
      </div>
      {expanded && children && (
        <div className="w5 bg-white ba b--sapphire mt1 fr z-999">
          <ul className="list pl0 mt0 mb0">{children(ExpansionItem)}</ul>
        </div>
      )}
    </div>
  );
};

export default ExpandButton;
