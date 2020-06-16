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
      <div key={`${style}-${size}`}>
        <h5 className="header5">
          {style} {size}
        </h5>
        <Button type={style} size={size}>
          Button
        </Button>
      </div>,
    );

    buttons.push(
      <div key={`${style}-${size}-disabled`}>
        <h5 className="header5">
          {style} {size} disabled
        </h5>
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
