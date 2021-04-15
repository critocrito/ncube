import c from "clsx";
import {useField} from "formik";
import React from "react";

import exclamationMark from "../svg/exclamation_mark.svg";

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
  const classes = c("block w-full pr-10 sm:text-sm p-2 border", {
    "border-error focus:outline-none focus:ring-error focus:border-error": hasError,
    "border-solitude": !hasError,
  });

  // FIXME: When I change the SVG implementation use text-error for the exclamation mark.
  return (
    <div>
      <div>
        <label htmlFor={name} className="block text-sm mb-1">
          {label}
        </label>
        <div className="mt-1 relative rounded-md shadow-sm">
          <input
            name={name}
            value={value}
            type={type}
            className={classes}
            placeholder={placeholder}
            disabled={disabled}
            onChange={onChange}
            onBlur={onBlur}
            aria-invalid="true"
            aria-describedby="email-error"
          />
          {hasError && (
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
              <img
                src={exclamationMark}
                className="h-5 w-5 text-error fill-current"
                aria-hidden="true"
              />
            </div>
          )}
        </div>
        {hasError && <p className="mt-2 text-sm text-error">{meta.error}</p>}
      </div>
    </div>
  );
};

export default Input;
