import React, {useEffect, useState} from "react";

import {useWorkspaceCtx} from "../lib/context";
import {statProcessesAll} from "../lib/http";
import {Process} from "../types";
import ProcessesActions from "./processes-actions";
import ProcessesInfoBox from "./processes-info-box";

interface ProcessesCardProps {
  process: Process;
  onShow: () => void;
  onRun: () => void;
}

const ProcessesCard = ({
  process: {key, name, config},
  onShow,
  onRun,
}: ProcessesCardProps) => {
  const [processesAll, setProcessesAll] = useState(0);

  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  useEffect(() => {
    const f = async () => {
      const stat = await statProcessesAll(slug, key);
      setProcessesAll(stat);
    };
    f();
  }, [slug, key]);

  const isSetup = config.reduce((memo, {value}) => {
    if (memo && value) return true;
    return false;
  }, true);

  return (
    <section className="h4 bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="flex w-80">
        <table className="w-100 collapse bn card">
          <colgroup>
            <col className="w-40" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-20" />
          </colgroup>

          <thead>
            <tr>
              <th className="bl br bt b--fair-pink tc b sapphire tl">
                <h4 className="header4 mt0 mb0 tl">{name}</h4>
              </th>
              <th className="ba b--fair-pink tc b sapphire">Selected</th>
              <th className="ba b--fair-pink tc b sapphire">All</th>
              <th className="ba b--fair-pink tc b sapphire">New</th>
            </tr>
          </thead>

          <tbody>
            <tr>
              <td className="bl br bb b--fair-pink tc sapphire tl">
                <ProcessesInfoBox isSetup={isSetup} onSetup={onShow} />
              </td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              <td className="ba b--fair-pink tc sapphire">
                {processesAll === 0 ? <>&mdash;</> : `${processesAll} sources`}
              </td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <ProcessesActions onRun={onRun} />
      </div>
    </section>
  );
};

export default ProcessesCard;
