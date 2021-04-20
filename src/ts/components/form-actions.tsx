import React from "react";

import ActionsLayout from "./actions-layout";
import Button from "./button";

interface FormActionsProps {
  submitLabel: string;
  onCancel?: () => void;
  isDisabled?: boolean;
}

const FormActions = ({
  submitLabel,
  isDisabled = false,
  onCancel,
}: FormActionsProps) => {
  return (
    <ActionsLayout align="right">
      {onCancel && (
        <Button type="reset" size="large" kind="secondary" onClick={onCancel}>
          Cancel
        </Button>
      )}
      <Button type="submit" size="large" disabled={isDisabled}>
        {submitLabel}
      </Button>
    </ActionsLayout>
  );
};

export default FormActions;
