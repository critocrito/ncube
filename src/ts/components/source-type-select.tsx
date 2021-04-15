/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import {useField, useFormikContext} from "formik";
import React from "react";
import {components, OptionProps, SingleValueProps} from "react-select";

import {SelectOption, SourceType} from "../types";
import FormLabel from "./form-label";
import SelectDropdown from "./select-dropdown";
import LabeledSourceTag from "./labeled-source-tag";

interface SourceTypeSelectProps {
  label: string;
  name: string;
  placeholder?: string;
  isClearable?: boolean;
}

const items: SourceType[] = [
  "youtube_video",
  "youtube_channel",
  "twitter_tweet",
  "twitter_user",
  "http_url",
];

const options: SelectOption[] = items.map((type) => ({
  value: type as string,
  label: type as string,
}));

export const SingleValue = <T extends SelectOption>(
  props: SingleValueProps<T>,
) => {
  const {data} = props;
  return (
    <components.SingleValue {...props} className="text-sapphire">
      <LabeledSourceTag label={data.label} />
    </components.SingleValue>
  );
};

export const Option = <T extends SelectOption, K extends boolean>({
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
      <LabeledSourceTag label={data.label} />
    </div>
  );
};

const SourceTypeSelect = ({
  label,
  isClearable = false,
  ...props
}: SourceTypeSelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(props);

  const {name, value} = field;
  const {touched, error} = meta;

  const hasError = touched && error;

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <FormLabel name={name} label={label} />

      <SelectDropdown<SelectOption>
        id="workspace-selector"
        options={options}
        defaultValue={value}
        onSelect={(option) => {
          if (!option) return;
          setFieldValue(field.name, option.value);
        }}
        isClearable={isClearable}
        LocalOption={Option}
        LocalSingleValue={SingleValue}
        className={
          "w-full rounded mb-2 cursor-pointer text-white border border-solitude"
        }
      />

      {hasError ? <div className="text-error">{meta.error}</div> : undefined}
    </div>
  );
};

export default SourceTypeSelect;
