import React from "react";

import Button from "./button";

interface FormActionsProps {
  submitLabel: string;
  onCancel?: () => void;
  isDisabled?: boolean;
}

const FormActionsProps = ({
  submitLabel,
  isDisabled = false,
  onCancel,
}: FormActionsProps) => {
  return (
    <div className="flex space-x-4 mt-3 ml-auto">
      {onCancel && (
        <Button type="reset" size="large" kind="secondary" onClick={onCancel}>
          Cancel
        </Button>
      )}
      <Button type="submit" size="large" disabled={isDisabled}>
        {submitLabel}
      </Button>
    </div>
  );
};

export default FormActionsProps;
