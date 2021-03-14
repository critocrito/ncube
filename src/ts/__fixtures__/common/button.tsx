import React from "react";

import Button from "../../common/button";

type Size = "normal" | "large";
type Kind = "primary" | "secondary" | "caution";

const sizes: Size[] = ["normal", "large"];
const kinds: Kind[] = ["primary", "secondary", "caution"];

const buttons: React.ReactElement[] = [];

kinds.forEach((kind) => {
  sizes.forEach((size) => {
    buttons.push(
      <div key={`${kind}-${size}`} className="flex items-center w-100">
        <p className="b w-30">
          {kind} {size}
        </p>
        <Button kind={kind} size={size}>
          Button
        </Button>
      </div>,
      <div key={`${kind}-${size}-disabled`} className="flex items-center w-100">
        <p className="b w-30">
          {kind} {size} disabled
        </p>
        <Button disabled kind={kind} size={size}>
          Button
        </Button>
      </div>,
    );
  });
});

export default (
  <div className="noto lh-copy pa2 flex flex-column">{buttons}</div>
);
