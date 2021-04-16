import React from "react";

import icon from "../svg/trash.svg";

interface TableDeleteRowProps {
  onDelete: () => void;
}

const TableDeleteRow = ({onDelete}: TableDeleteRowProps) => {
  return (
    <div
      className="trash"
      onClick={onDelete}
      onKeyPress={onDelete}
      role="button"
      tabIndex={0}
    >
      <img
        className="text-gray-light"
        height="100%"
        width="100%"
        src={icon}
        alt="Remove this row item."
      />
    </div>
  );
};

export default TableDeleteRow;
