import React from "react";

import Button from "./button";
import ActionsLayout from "./actions-layout";

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

export default FormActionsProps;
