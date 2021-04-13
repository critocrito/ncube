import React from "react";

import SegmentsEmpty from "../../mdx/segments-empty.mdx";
import {voidFn} from "../lib/utils";
import {Segment, Workspace} from "../types";
import DataCard from "./data-card";
import DataSegmentsTable from "./data-segments-table";
import IntroText from "./intro-text";

interface DataSegmentsHomeProps {
  workspace: Workspace;
  segments: Segment[];
  onShow?: () => void;
  onShowSegment?: (s: Segment) => void;
  onVerifySegment?: (s: Segment) => void;
  onDeleteSegment?: (s: Segment) => void;
}

const DataSegmentsHome = ({
  workspace,
  segments,
  onShow = voidFn,
  onShowSegment = voidFn,
  onVerifySegment = voidFn,
  onDeleteSegment = voidFn,
}: DataSegmentsHomeProps) => {
  return (
    <>
      <DataCard workspace={workspace} onShow={onShow} />

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
    </>
  );
};

export default DataSegmentsHome;
