/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import React from "react";
import Select, {
  components,
  ControlProps,
  GroupProps,
  InputProps,
  OptionProps,
  PlaceholderProps,
  SingleValueProps,
  ValueType,
  IndicatorProps,
  MenuListComponentProps,
} from "react-select";

import {SelectOption} from "../types";

export const Input = (props: InputProps) => {
  return <components.Input {...props} className="text-base text-white" />;
};

export const LargeInput = (props: InputProps) => {
  return (
    <components.Input {...props} className="text-lg leading-none text-white" />
  );
};

export const SingleValue = <T extends SelectOption>(
  props: SingleValueProps<T>,
) => {
  console.log(props);
  return <components.SingleValue {...props} className="font-bold text-white" />;
};

export const LargeSingleValue = <T extends SelectOption>({
  children,
  innerProps,
}: SingleValueProps<T>) => {
  return (
    <div className="text-lg flex-1 leading-none" {...innerProps}>
      {children}
    </div>
  );
};

export const IndicatorSeparator = () => {
  return <span />;
};

export const DropdownIndicator = <T extends SelectOption, K extends boolean>(
  props: IndicatorProps<T, K>,
) => {
  return (
    <components.DropdownIndicator
      className="py-0 px-2 text-white fill-current focus:text-white"
      {...props}
    />
  );
};

export const Control = <T extends SelectOption, K extends boolean>({
  children,
  innerRef,
  innerProps,
}: ControlProps<T, K>) => {
  return (
    <div
      className="flex flex-row justify-between items-center"
      ref={innerRef}
      {...innerProps}
    >
      {children}
    </div>
  );
};

export const Group = <T extends SelectOption, K extends boolean>({
  Heading,
  children,
  isMulti,
  ...props
}: GroupProps<T, K>) => {
  const className = {
    "flex flex-wrap": isMulti,
  };

  return (
    <div className="p-2 canvas">
      <Heading {...props} />
      <div className={c("bg-canvas mt-1", className)}>{children}</div>
    </div>
  );
};

export const Placeholder = <T extends SelectOption, K extends boolean>({
  children,
  innerProps,
}: PlaceholderProps<T, K>) => {
  return (
    <div className="" {...innerProps}>
      {children}
    </div>
  );
};

export const MenuList = <T extends SelectOption, K extends boolean>(
  props: MenuListComponentProps<T, K>,
) => {
  return <components.MenuList className="bg-canvas rounded-md" {...props} />;
};

export const Option = <T extends SelectOption, K extends boolean>({
  isSelected,
  isFocused,
  innerRef,
  innerProps,
  data,
}: OptionProps<T, K>) => {
  const className = c("px-2 py-1", {
    "bg-fair-pink text-sapphire": isSelected || isFocused,
    "cursor-pointer": !isSelected,
    "bg-canvas text-sapphire": !isFocused && !isSelected,
  });

  return (
    <div className={className} ref={innerRef} {...innerProps}>
      {data.label}
    </div>
  );
};

interface SelectorProps<T extends SelectOption> {
  id: string;
  options: T[];
  onSelect: (value?: T) => void;
  defaultValue?: T;
  isSearchable?: boolean;
  isClearable?: boolean;
  LocalOption?: (props: OptionProps<T, false>) => JSX.Element;
  LocalSingleValue?: (props: SingleValueProps<T>) => JSX.Element;
  className?: string;
}

const SelectDropdown = <T extends SelectOption>({
  id,
  options,
  onSelect,
  defaultValue,
  isSearchable = false,
  isClearable = false,
  LocalOption = Option,
  LocalSingleValue = SingleValue,
  className,
}: SelectorProps<T>) => {
  const handleChange = (value: ValueType<T, false>) => {
    onSelect(value || undefined);
  };

  return (
    <Select
      instanceId={id}
      options={options}
      className={c("w-full", className)}
      defaultValue={defaultValue}
      openMenuOnFocus
      isSearchable={isSearchable}
      isClearable={isClearable}
      components={{
        Input,
        IndicatorSeparator,
        Placeholder,
        Control,
        DropdownIndicator,
        MenuList,
        SingleValue: LocalSingleValue,
        Option: LocalOption,
      }}
      onChange={handleChange}
    />
  );
};

export default SelectDropdown;
