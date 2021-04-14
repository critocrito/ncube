import React from "react";

import Stat from "./stat-dashboard";

interface DashboardStatsProps {
  fetchSourcesStat: () => Promise<number>;
  fetchDataStat: () => Promise<number>;
  fetchInvestigationsStat: () => Promise<number>;
}

const DashboardStats = ({
  fetchSourcesStat,
  fetchDataStat,
  fetchInvestigationsStat,
}: DashboardStatsProps) => {
  return (
    <div className="flex items-center space-x-20 md:space-x-12">
      <Stat kind="source" fetchStat={fetchSourcesStat} />
      <Stat kind="data" fetchStat={fetchDataStat} />
      <Stat kind="investigation" fetchStat={fetchInvestigationsStat} />
    </div>
  );
};

export default DashboardStats;
