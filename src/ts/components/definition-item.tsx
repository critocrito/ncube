import React from "react";

interface DefinitionItemProps {
  item: string;
  value: string | JSX.Element;
}

const DefinitionItem = ({item, value}: DefinitionItemProps) => {
  return (
    <dl className="mv2">
      <dt className="dib b text-md">{item}:</dt>
      <dd className="dib ml1 text-md">{value}</dd>
    </dl>
  );
};

export default DefinitionItem;
