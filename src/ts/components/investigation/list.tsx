import React from "react";

import {Investigation} from "../../types";
import InvestigationCard from "./card";

interface InvestigationListProps {
  investigations: Investigation[];
  onClick: (p: Investigation) => void;
}

const InvestigationList = ({
  investigations,
  onClick,
}: InvestigationListProps) => {
  return (
    <div className="flex flex-column">
      {investigations.map((investigation) => (
        <InvestigationCard
          key={investigation.id}
          investigation={investigation}
          onClick={() => onClick(investigation)}
        />
      ))}
    </div>
  );
};

export default InvestigationList;
