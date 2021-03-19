import React from "react";

import icon from "../../../../resources/public/images/icon_trash.svg";

interface DeleteRowProps {
  onDelete: () => void;
}

const DeleteRow = ({onDelete}: DeleteRowProps) => {
  return (
    <div
      className="trash"
      onClick={onDelete}
      onKeyPress={onDelete}
      role="button"
      tabIndex={0}
    >
      <img
        className="gray-25"
        height="100%"
        width="100%"
        src={icon}
        alt="Remove this row item."
      />
    </div>
  );
};

export default DeleteRow;
