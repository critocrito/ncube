import {useField, useFormikContext} from "formik";
import React from "react";

import {SelectOption} from "../types";
import FormLabel from "./form-label";
import SelectDropdown from "./select-dropdown";

interface SelectProps {
  label: string;
  name: string;
  options: SelectOption[];
  placeholder?: string;
}

const OptionsSelect = ({label, options, ...props}: SelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value} = field;
  const {touched, error} = meta;
  const hasError = touched && error;

  return (
    <div>
      <FormLabel name={name} label={label} />

      <SelectDropdown<SelectOption>
        id="workspace-selector"
        options={options}
        defaultValue={{label: name, value}}
        onSelect={(option) => {
          if (!option) return;
          setFieldValue(field.name, option.value);
        }}
        isClearable
        className="w-full mb-2 cursor-pointer border border-solitude"
      />

      {hasError ? <div className="text-error">{meta.error}</div> : undefined}
    </div>
  );
};

export default OptionsSelect;
