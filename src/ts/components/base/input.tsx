import React from "react";

interface InputProps {
  label: string;
  name: string;
  onChange?: (value: string | undefined) => void;
  placeholder?: string;
  value?: string;
  disabled?: boolean;
}

const Input = ({
  placeholder = "",
  value = "",
  disabled = false,
  onChange = (_value: string | undefined) => {},
  name,
  label,
}: InputProps) => {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const {value: newValue}: {value: string} = e.target;
    onChange(newValue === "" ? undefined : newValue);
  };

  return (
    <div className="flex flex-column fb1 mt3 mb2 w-two-thirds">
      <label htmlFor={name} className="mb1">
        {label}
      </label>
      <input
        className="fb1 pa2 ba b--barrier"
        name={name}
        value={value}
        disabled={disabled}
        placeholder={placeholder}
        onChange={handleChange}
      />
    </div>
  );
};

export default Input;
