import React from "react";

interface PanelHeaderProps {
  header: string;
  description?: string;
}

const PanelHeader = ({header, description}: PanelHeaderProps) => {
  return (
    <div>
      <h1 className="header1">{header}</h1>
      {description && <p className="my-4">{description}</p>}
    </div>
  );
};

export default PanelHeader;
