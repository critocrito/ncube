import React from "react";

import {Download} from "../types";
import {capitalize} from "../utils";

interface MediaDefinitionProps {
  downloads: Download[];
}

const MediaDefinition = ({downloads}: MediaDefinitionProps) => {
  const media = ["video", "image"].reduce((memo, type) => {
    const xs = downloads.filter(({type: t}) => t === type);
    if (xs.length === 0) return memo;
    return memo.concat([`${xs.length} ${capitalize(type)}s`]);
  }, [] as string[]);

  return <span>{media.join(", ")}</span>;
};

export default MediaDefinition;
