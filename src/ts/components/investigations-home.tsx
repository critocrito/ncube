import React from "react";

import {voidFn} from "../lib/utils";
import {Investigation} from "../types";
import InvestigationsActions from "./investigations-actions";
import InvestigationsTable from "./investigations-table";
import ActionsLayout from "./actions-layout";

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
    <div className="flex flex-col space-y-3">
      <ActionsLayout>
        <InvestigationsActions onCreate={onCreate} />
      </ActionsLayout>

      <InvestigationsTable onShow={onShow} investigations={investigations} />
    </div>
  );
};

export default InvestigationsHome;
