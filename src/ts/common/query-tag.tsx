import React from "react";

interface QueryTagProps {
  label: string;
}

const QueryTag = ({label}: QueryTagProps) => {
  return (
    <div className="tag flex flex-column justify-around bg-gray-25 nowrap">
      <span>{label}</span>
    </div>
  );
};

export default QueryTag;
