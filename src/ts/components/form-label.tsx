import React from "react";

interface FormLabelProps {
  name: string;
  label: string;
}

const FormLabel = ({name, label}: FormLabelProps) => {
  return (
    <label htmlFor={name} className="block text-sapphire font-bold mb-1">
      {label}
    </label>
  );
};

export default FormLabel;
