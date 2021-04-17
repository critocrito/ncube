import React from "react";

import {Investigation} from "../types";
import InvestigationsCard from "./investigations-card";

interface InvestigationsTableProps {
  investigations: Investigation[];
  onShow: (p: Investigation) => void;
}

const InvestigationsTable = ({
  investigations,
  onShow,
}: InvestigationsTableProps) => {
  return (
    <div className="flex flex-col">
      {investigations.map((investigation) => (
        <InvestigationsCard
          key={investigation.id}
          investigation={investigation}
          onShow={() => onShow(investigation)}
        />
      ))}
    </div>
  );
};

export default InvestigationsTable;
