import React from "react";

import Button from "../base/button";

type Size = "normal" | "large";
type Style = "primary" | "secondary" | "caution";

const sizes: Size[] = ["normal", "large"];
const styles: Style[] = ["primary", "secondary", "caution"];

const buttons: React.ReactElement[] = [];

styles.forEach((style) => {
  sizes.forEach((size) => {
    buttons.push(
      <div key={`${style}-${size}`} className="flex items-center w-100">
        <p className="b w-30">
          {style} {size}
        </p>
        <Button type={style} size={size}>
          Button
        </Button>
      </div>,
    );

    buttons.push(
      <div
        key={`${style}-${size}-disabled`}
        className="flex items-center w-100"
      >
        <p className="b w-30">
          {style} {size} disabled
        </p>
        <Button disabled type={style} size={size}>
          Button
        </Button>
      </div>,
    );
  });
});

export default (
  <div className="noto lh-copy pa2 flex flex-column">{buttons}</div>
);
