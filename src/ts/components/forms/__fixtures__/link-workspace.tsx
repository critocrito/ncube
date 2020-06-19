import React, {useState} from "react";

import LinkWorkspace, {LinkWorkspaceFormValues} from "../link-workspace";

const Wrapper = () => {
  const [state, setState] = useState<LinkWorkspaceFormValues | undefined>();
  const [isCanceled, setIsCanceled] = useState<boolean>(false);

  const initialValues = {
    workspace: "syrian-archive",
    name: "Syrian Archive",
    description: "",
    endpoint: "https://syrian-archive.org/ncube",
    email: "me@example.org",
    otp:
      "aes256cbc$xQqdV4FqxcivzACEs6cBAA==$3Nac+j09z28xPHSXquWeVa2kpbFHn+aeTwhA1nfW+qUAAxi33JgI3JGvUb7dGXrVHcFOPw6BWMBrZxZCUUt3FRXFDN1I1yFY41lD4+/4Efg=",
    password: "",
    password_again: "",
  };

  const handleSubmit = (values: LinkWorkspaceFormValues) => {
    setIsCanceled(false);
    setState(values);
  };
  const handleCancel = () => setIsCanceled(true);

  return (
    <div>
      <LinkWorkspace
        initialValues={{database: "http", kind: "remote", ...initialValues}}
        onCancel={handleCancel}
        onSubmit={handleSubmit}
      />
      {state === undefined ? (
        ""
      ) : (
        <div>
          <p>Submitted: {JSON.stringify(state)}</p>
        </div>
      )}
      {isCanceled ? (
        <div>
          <p>Form got canceled.</p>
        </div>
      ) : undefined}
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper />
  </div>
);
