/* eslint react/jsx-props-no-spreading: off */
import "react-datepicker/dist/react-datepicker.css";

import {useField, useFormikContext} from "formik";
import React from "react";
import D from "react-datepicker";
import FormLabel from "./form-label";

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
    <div>
      <FormLabel name={name} label={label} />

      <D
        {...field}
        {...props}
        className="border border-solitude p-3"
        selected={(field.value && new Date(field.value)) || undefined}
        onChange={(val) => {
          setFieldValue(field.name, val);
        }}
        showTimeSelect
        dateFormat="Pp"
      />

      {hasError && <div className="error">{meta.error}</div>}
    </div>
  );
};

export default DatePicker;
