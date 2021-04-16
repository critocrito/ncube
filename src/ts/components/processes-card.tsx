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
    <section className="h-36 bg-white p-8 shadow-md flex items-center">
      <table className="w-full h-full">
        <colgroup>
          <col className="w-2/5" />
          <col className="w-1/5" />
          <col className="w-1/5" />
          <col className="w-1/5" />
        </colgroup>

        <thead>
          <tr>
            <th className="border border-fair-pink font-bold text-sapphire p-2">
              <h4 className="header4 text-left">{name}</h4>
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Selected
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              All
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              New
            </th>
          </tr>
        </thead>

        <tbody>
          <tr>
            <td className="border-l border-b border-r border-fair-pink text-sapphire">
              <ProcessesInfoBox isSetup={isSetup} onSetup={onShow} />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              &mdash;
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              {processesAll === 0 ? <>&mdash;</> : `${processesAll} sources`}
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              &mdash;
            </td>
          </tr>
        </tbody>
      </table>

      <div className="pr-3 h-full ml-auto">
        <ProcessesActions onRun={onRun} />
      </div>
    </section>
  );
};

export default ProcessesCard;
