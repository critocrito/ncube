/* eslint react/jsx-props-no-spreading: off */
import "react-datepicker/dist/react-datepicker.css";

import {useField, useFormikContext} from "formik";
import React from "react";
import D from "react-datepicker";

interface DatePickerProps {
  label: string;
  name: string;
}

const DatePicker = ({label, ...props}: DatePickerProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name} = field;
  const {touched, error} = meta;
  const hasError = touched && error;

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <label htmlFor={name} className="mb1">
        {label}
      </label>

      <D
        {...field}
        {...props}
        className="ba b--solitude pa2"
        selected={(field.value && new Date(field.value)) || undefined}
        onChange={(val) => {
          setFieldValue(field.name, val);
        }}
        showTimeSelect
        dateFormat="Pp"
      />

      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default DatePicker;
