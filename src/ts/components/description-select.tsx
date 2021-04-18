/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import {useField, useFormikContext} from "formik";
import React from "react";
import {OptionProps} from "react-select";

import {DescriptionOption} from "../types";
import FormLabel from "./form-label";
import SelectDropdown from "./select-dropdown";

interface DescriptionSelectProps {
  options: DescriptionOption[];
  label: string;
  name: string;
  placeholder?: string;
  isClearable?: boolean;
}

export const Option = <T extends DescriptionOption, K extends boolean>({
  isSelected,
  isFocused,
  innerRef,
  innerProps,
  data,
}: OptionProps<T, K>) => {
  const className = c("flex flex-col px-2 py-1", {
    "bg-fair-pink text-sapphire": isSelected || isFocused,
    "cursor-pointer": !isSelected,
    "bg-canvas text-sapphire": !isFocused && !isSelected,
  });

  return (
    <div className={className} ref={innerRef} {...innerProps}>
      {data.label}
      {data.description && <span className="text-sm">{data.description}</span>}
    </div>
  );
};

const DescriptionSelect = ({
  options,
  label,
  isClearable = false,
  ...props
}: DescriptionSelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value} = field;
  const {touched, error} = meta;

  const hasError = touched && error;

  return (
    <div>
      <FormLabel name={name} label={label} />

      <SelectDropdown<DescriptionOption>
        id="workspace-selector"
        options={options}
        defaultValue={value}
        onSelect={(option) => {
          if (!option) return;
          setFieldValue(field.name, option.value);
        }}
        isClearable={isClearable}
        LocalOption={Option}
        className={"w-full rounded cursor-pointer border border-solitude"}
      />

      {hasError ? <div className="text-error">{meta.error}</div> : undefined}
    </div>
  );
};

export default DescriptionSelect;
