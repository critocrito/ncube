import React, {useState} from "react";

import Input from "../base/input";

interface WrapperProps {
  label: string;
  placeholder?: string;
  disabled?: boolean;
}
const Wrapper = ({label, placeholder = "", disabled = false}: WrapperProps) => {
  const [state, setState] = useState<string | undefined>();
  const name = label.replace(" ", "-");

  return (
    <Input
      name={name}
      label={label}
      placeholder={placeholder}
      disabled={disabled}
      value={state}
      onChange={setState}
    />
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper label="Standard Input" />
    <Wrapper
      label="Input with Placeholder"
      placeholder="Type something here."
    />
    <Wrapper
      disabled
      label="Disabled Input"
      placeholder="Type something here."
    />
  </div>
);
