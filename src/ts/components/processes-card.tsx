import React, {useEffect, useState} from "react";

import {useWorkspaceCtx} from "../lib/context";
import {statProcessesAll} from "../lib/http";
import {Process} from "../types";
import ProcessesActions from "./processes-actions";
import ProcessesInfoBox from "./processes-info-box";
import Card from "./card";

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
    <Card>
      <table className="w-full h-full max-w-xl">
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
            <td className="border-l border-b border-r border-fair-pink text-sapphire py-4">
              <ProcessesInfoBox isSetup={isSetup} onSetup={onShow} />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              &mdash;
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              {processesAll === 0 ? <>&mdash;</> : processesAll}
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              &mdash;
            </td>
          </tr>
        </tbody>
      </table>

      <div className="self-start ml-auto">
        <ProcessesActions onRun={onRun} />
      </div>
    </Card>
  );
};

export default ProcessesCard;
