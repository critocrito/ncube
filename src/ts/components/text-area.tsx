import c from "classnames";
import {useField} from "formik";
import React from "react";

interface TextareaProps {
  label: string;
  name: string;
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onBlur?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  placeholder?: string;
  disabled?: boolean;
}

const Textarea = ({
  label,
  placeholder = "",
  disabled = false,
  ...props
}: TextareaProps) => {
  const fieldProps = {
    placeholder,
    ...props,
  };
  const [field, meta] = useField(fieldProps);

  const {name, value, onChange, onBlur} = field;

  const {touched, error} = meta;

  const hasError = touched && error;
  const classes = c("fb1 pa2 ba", hasError ? "b--error" : "b--solitude");

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <label htmlFor={name} className="mb1">
        {label}
      </label>
      <textarea
        className={classes}
        name={name}
        value={value}
        placeholder={placeholder}
        disabled={disabled}
        onChange={onChange}
        onBlur={onBlur}
      />
      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default Textarea;
