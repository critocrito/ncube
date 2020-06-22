import {Form, Formik} from "formik";
import React from "react";

import Input from "../input";

interface WrapperProps {
  label: string;
  placeholder?: string;
  disabled?: boolean;
}
const Wrapper = ({label, placeholder = "", disabled = false}: WrapperProps) => {
  return (
    <Formik initialValues={{field: ""}} onSubmit={() => {}}>
      <Form>
        <Input
          name="field"
          label={label}
          placeholder={placeholder}
          disabled={disabled}
        />
      </Form>
    </Formik>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper label="Standard Input" />
    <Wrapper
      label="Input with Placeholder"
      placeholder="Type something here."
    />
    <Wrapper disabled label="Disabled Input" placeholder="Can't type here." />
  </div>
);
