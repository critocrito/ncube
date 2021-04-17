import React from "react";

import SegmentsEmpty from "../../mdx/segments-empty.mdx";
import {voidFn} from "../lib/utils";
import {Segment, Workspace, DataStats} from "../types";
import DataCard from "./data-card";
import DataSegmentsTable from "./data-segments-table";
import IntroText from "./intro-text";

interface DataSegmentsHomeProps {
  workspace: Workspace;
  segments: Segment[];
  stats: DataStats;
  onShow?: () => void;
  onShowSegment?: (s: Segment) => void;
  onVerifySegment?: (s: Segment) => void;
  onDeleteSegment?: (s: Segment) => void;
}

const DataSegmentsHome = ({
  workspace,
  segments,
  stats,
  onShow = voidFn,
  onShowSegment = voidFn,
  onVerifySegment = voidFn,
  onDeleteSegment = voidFn,
}: DataSegmentsHomeProps) => {
  return (
    <div className="space-y-8">
      <DataCard stats={stats} onShow={onShow} />

      {segments.length > 0 ? (
        <DataSegmentsTable
          workspace={workspace}
          segments={segments}
          onShow={onShowSegment}
          onVerify={onVerifySegment}
          onDelete={onDeleteSegment}
        />
      ) : (
        <IntroText>
          <SegmentsEmpty />
        </IntroText>
      )}
    </div>
  );
};

export default DataSegmentsHome;
