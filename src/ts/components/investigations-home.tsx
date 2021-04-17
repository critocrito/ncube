import React from "react";

import {voidFn} from "../lib/utils";
import {Investigation} from "../types";
import InvestigationsActions from "./investigations-actions";
import InvestigationsTable from "./investigations-table";

interface InvestigationsHomeProps {
  investigations: Investigation[];
  onCreate?: () => void;
  onShow?: (investigation: Investigation) => void;
}

const InvestigationsHome = ({
  investigations,
  onCreate = voidFn,
  onShow = voidFn,
}: InvestigationsHomeProps) => {
  return (
    <div className="flex flex-col">
      <InvestigationsActions onCreate={onCreate} />

      <InvestigationsTable onShow={onShow} investigations={investigations} />
    </div>
  );
};

export default InvestigationsHome;
