import React, {useEffect, useState} from "react";

import DataDetails from "../database/details";
import {showUnit} from "../http";
import {Unit, Workspace} from "../types";

interface VerificationDetailsProps {
  workspace: Workspace;
  unitId: number;
}

const VerificationDetails = ({
  workspace: {slug},
  unitId,
}: VerificationDetailsProps) => {
  const [unit, setUnit] = useState<Unit | undefined>();

  useEffect(() => {
    const f = async () => {
      const data = await showUnit(slug, unitId);
      setUnit(data);
    };
    f();
  }, [slug, unitId]);

  return (
    <div className="flex">
      {unit && (
        <div className="w-50">
          <DataDetails unit={unit} />
        </div>
      )}
      {unit && <div className="w-50">Annotations</div>}
    </div>
  );
};

export default VerificationDetails;
