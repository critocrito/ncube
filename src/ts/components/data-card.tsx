import React from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import {DataStats, Workspace} from "../types";
import Button from "./button";
import StatsTable from "./stats-table";

interface DataCardProps {
  workspace: Workspace;
  onShow: () => void;
  stats: DataStats;
}

const DataCard = ({workspace: {slug}, stats, onShow}: DataCardProps) => {
  return (
    <div className="h-36 bg-white p-8 shadow-md flex items-center">
      <div className="w-64 flex flex-col justify-between h-full">
        <div className="flex items-center pb-1.5">
          <img
            src={dataIcon}
            className="h-8 w-8"
            alt={"Icon for the data section."}
          />
          <h4 className="header4 pl-2">Data</h4>
        </div>
        <p className="text-sm">All the data.</p>
      </div>

      <div className="w-72 h-full ml-4">
        {stats && <StatsTable stats={stats} />}
      </div>

      <div className="pr-3 h-full ml-auto">
        <Button onClick={onShow}>Explore</Button>
      </div>
    </div>
  );
};

export default DataCard;
