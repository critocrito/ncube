import {useField, useFormikContext} from "formik";
import React from "react";

import {SelectOption} from "../types";
import FormLabel from "./form-label";
import SelectDropdown from "./select-dropdown";

interface SelectProps {
  label: string;
  name: string;
  placeholder?: string;
  isClearable?: boolean;
}

const YES: SelectOption = {value: "yes", label: "Yes"};
const NO: SelectOption = {value: "no", label: "No"};

const options: SelectOption[] = [YES, NO];

const BooleanSelect = ({label, isClearable = false, ...props}: SelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value} = field;
  const {touched, error} = meta;
  const hasError = touched && error;

  let optionValue: SelectOption | undefined;

  if (value) {
    optionValue = YES;
  } else if (typeof value === "boolean" && !value) {
    optionValue = NO;
  }

  return (
    <div>
      <FormLabel name={name} label={label} />

      <SelectDropdown<SelectOption>
        id="workspace-selector"
        options={options}
        defaultValue={optionValue}
        onSelect={(option) => {
          if (!option) return;
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
        isClearable={isClearable}
        className={"w-full rounded cursor-pointer border border-solitude"}
      />

      {hasError ? <div className="text-error">{meta.error}</div> : undefined}
    </div>
  );
};

export default BooleanSelect;
