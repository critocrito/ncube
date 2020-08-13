import React, {useEffect, useState} from "react";

import LoadingSpinner from "./loading-spinner";

interface StatProps {
  fetchStat: () => Promise<number>;
  suffix?: string;
}

const Stat = ({fetchStat, suffix = ""}: StatProps) => {
  const [fetchDone, setFetchDone] = useState(false);
  const [statValue, setStatValue] = useState(0);

  useEffect(() => {
    const f = async () => {
      const stat = await fetchStat();
      setStatValue(stat);
      setFetchDone(true);
    };
    f();
  }, [fetchStat]);

  if (!fetchDone) return <LoadingSpinner />;

  return statValue === 0 ? <>&mdash;</> : <>{`${statValue} ${suffix}`}</>;
};

export default Stat;
