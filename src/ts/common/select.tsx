import {useField, useFormikContext} from "formik";
import React from "react";
import S, {OptionTypeBase as Option} from "react-select";

import styles from "./select-styles";

interface SelectProps {
  label: string;
  name: string;
  options: Option[];
}

const Select = ({label, options, ...props}: SelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value, onBlur} = field;
  const {touched, error} = meta;
  const hasError = touched && error;

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <label htmlFor={name} className="mb1">
        {label}
      </label>
      <S
        styles={styles}
        options={options}
        name={name}
        value={options ? options.find((option) => option?.value === value) : ""}
        onChange={(option: Option) =>
          setFieldValue(field.name, option?.value || "")
        }
        onBlur={onBlur}
        isClearable
        isSearchable
      />

      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default Select;
