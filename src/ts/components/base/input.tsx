import c from "classnames";
import {useField} from "formik";
import React from "react";

interface InputProps {
  label: string;
  name: string;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onBlur?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
  disabled?: boolean;
  type?: "text" | "email" | "password";
}

const Input = ({
  label,
  placeholder = "",
  disabled = false,
  type = "text",
  ...props
}: InputProps) => {
  const fieldProps = {
    placeholder,
    type,
    ...props,
  };
  const [field, meta] = useField(fieldProps);

  const {name, value, onChange, onBlur} = field;
  const {touched, error} = meta;

  const hasError = touched && error;
  const classes = c("fb1 pa2 ba", hasError ? "b--error" : "b--barrier");

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <label htmlFor={name} className="mb1">
        {label}
      </label>
      <input
        className={classes}
        name={name}
        value={value}
        type={type}
        placeholder={placeholder}
        disabled={disabled}
        onChange={onChange}
        onBlur={onBlur}
      />
      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default Input;
