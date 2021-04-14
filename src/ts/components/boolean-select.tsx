import {useField, useFormikContext} from "formik";
import React from "react";
import S, {OptionTypeBase as Option} from "react-select";

import styles from "./select-styles";

interface SelectProps {
  label: string;
  name: string;
  description?: string;
  placeholder?: string;
  isClearable?: boolean;
}

const YES: Option = {value: "yes", label: "Yes"};
const NO: Option = {value: "no", label: "No"};

const options: Option[] = [YES, NO];

const BooleanSelect = ({
  label,
  description,
  isClearable = true,
  ...props
}: SelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value, onBlur} = field;
  const {touched, error} = meta;
  const hasError = touched && error;

  let optionValue: Option | undefined;

  if (value) {
    optionValue = YES;
  } else if (typeof value === "boolean" && !value) {
    optionValue = NO;
  }

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <label htmlFor={name} className="mb1 b sapphire">
        {label}
      </label>

      {description && <span className="text-md mb2">{description}</span>}

      <S
        styles={styles}
        options={options}
        name={name}
        value={optionValue}
        onChange={(option: Option) => {
          let val;

          if (option?.value === "yes") {
            val = true;
          } else if (option?.value === "no") {
            val = false;
          } else {
            val = undefined;
          }

          setFieldValue(field.name, val);
        }}
        onBlur={onBlur}
        isClearable={isClearable}
      />

      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default BooleanSelect;
