/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import {useField, useFormikContext} from "formik";
import React from "react";
import {
  ActionMeta,
  components,
  ControlProps,
  MenuListComponentProps,
  MultiValueProps,
  OptionProps,
  ValueContainerProps,
  ValueType,
} from "react-select";
import CreatableSelect from "react-select/creatable";

import {DescriptionOption} from "../types";
import FormLabel from "./form-label";
import QueryTag from "./query-tag";

interface SourceTagSelectProps {
  options: DescriptionOption[];
  label: string;
  name: string;
  isClearable?: boolean;
  className?: string;
}

const MenuList = (props: MenuListComponentProps<DescriptionOption, true>) => {
  return <components.MenuList className="bg-white space-y-1" {...props} />;
};

const Control = (props: ControlProps<DescriptionOption, true>) => (
  <components.Control className="space-x-4 border border-solitude" {...props} />
);

const MultiValue = ({
  data: {label},
  innerProps,
}: MultiValueProps<DescriptionOption>) => {
  return <QueryTag {...innerProps} label={label} />;
};

const Option = ({
  children: _children,
  ...props
}: OptionProps<DescriptionOption, true>) => {
  const {
    isSelected,
    isFocused,
    innerRef,
    innerProps,
    data: {label},
  } = props;

  const className = c("px-1.5 py-2 cursor-pointer", {
    "bg-canvas": isFocused && !isSelected,
  });

  return (
    <div className={className} ref={innerRef} {...innerProps}>
      <QueryTag label={label} />
    </div>
  );
};

const ValueContainer = (
  props: ValueContainerProps<DescriptionOption, true>,
) => {
  return <components.ValueContainer {...props} className="space-x-2" />;
};

const SourceTagSelect = ({
  options,
  label,
  name,
  isClearable = true,
  className,
}: SourceTagSelectProps) => {
  const {setFieldValue} = useFormikContext();
  const [field, meta] = useField(name);

  const handleSelect = (
    value: ValueType<DescriptionOption, true>,
    {action}: ActionMeta<DescriptionOption>,
  ) => {
    // eslint-disable-next-line default-case
    switch (action) {
      case "select-option": {
        if (value) {
          setFieldValue(name, value);
        }

        break;
      }
      case "deselect-option": {
        if (value) {
          setFieldValue(name, value);
        }

        break;
      }
      case "clear": {
        setFieldValue(name, []);
        break;
      }
    }
  };
  const {touched, error} = meta;

  const hasError = touched && error;

  const handleCreate = (tag: string) => {
    setFieldValue(name, [...field.value, {label: tag, value: tag}]);
  };

  return (
    <div className={className}>
      <FormLabel name={name} label={label} />

      <CreatableSelect
        options={options}
        value={field.value}
        onChange={handleSelect}
        onCreateOption={handleCreate}
        components={{
          Option,
          Control,
          MenuList,
          MultiValue,
          ValueContainer,
        }}
        closeMenuOnSelect={false}
        isClearable={isClearable}
        isMulti
      />

      {hasError ? <div className="text-error">{meta.error}</div> : undefined}
    </div>
  );
};

export default SourceTagSelect;
