import React from "react";

import {Investigation} from "../../types";
import Button from "../button";
import InvestigationList from "./list";

interface InvestigationHomeProps {
  investigations: Investigation[];
  onCreate: () => void;
  onShow: (investigation: Investigation) => void;
}

const InvestigationHome = ({
  investigations,
  onCreate,
  onShow,
}: InvestigationHomeProps) => {
  return (
    <div className="flex flex-column">
      <div className="flex mb3">
        <Button className="ml-auto" size="large" onClick={onCreate}>
          Create New
        </Button>
      </div>

      <InvestigationList onClick={onShow} investigations={investigations} />
    </div>
  );
};

export default InvestigationHome;
