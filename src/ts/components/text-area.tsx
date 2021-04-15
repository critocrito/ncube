import c from "clsx";
import {useField} from "formik";
import React from "react";

import FormLabel from "./form-label";

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
  const classes = c("block w-full pr-10 sm:text-sm p-2 border", {
    "border-error focus:outline-none focus:ring-error focus:border-error": hasError,
    "border-solitude": !hasError,
  });

  return (
    <div>
      <FormLabel name={name} label={label} />

      <textarea
        className={classes}
        name={name}
        value={value}
        placeholder={placeholder}
        disabled={disabled}
        onChange={onChange}
        onBlur={onBlur}
        rows={3}
      />
      {hasError && <div className="text-error">{meta.error}</div>}
    </div>
  );
};

export default Textarea;
