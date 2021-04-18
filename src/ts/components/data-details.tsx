import {format, parseISO} from "date-fns";
import React from "react";

import {capitalize} from "../lib/utils";
import {Download, Unit} from "../types";
import MediaViewer from "./media-viewer";
import QueryTagList from "./query-tag-list";
import SourceTag from "./source-tag";
import Description from "./description";

interface DataDetailsProps {
  unit: Unit;
}

const mediaCounts = (downloads: Download[]): string => {
  const media = ["video", "image"].reduce((memo, type) => {
    const xs = downloads.filter(({type: t}) => t === type);
    if (xs.length === 0) return memo;
    return [...memo, `${xs.length} ${capitalize(type)}s`];
  }, [] as string[]);

  return media.join(", ");
};

const DataDetails = ({unit}: DataDetailsProps) => {
  let kind: "youtube" | "twitter" | "http";
  switch (true) {
    case unit.source.startsWith("youtube"): {
      kind = "youtube";
      break;
    }
    case unit.source.startsWith("twitter"): {
      kind = "twitter";
      break;
    }
    default:
      kind = "http";
  }

  const createdAt = unit.created_at
    ? format(parseISO(unit.created_at), "yyyy-MM-dd")
    : "";
  const fetchedAt = format(parseISO(unit.fetched_at), "yyyy-MM-dd");

  const items = [
    {
      label: "Type of unit",
      value: (
        <div className="flex items-center">
          <SourceTag kind={kind} />
          <span className="ml-3">{unit.source}</span>
        </div>
      ),
    },
    {label: "Title", value: unit.title},
    {label: "URL", value: unit.href},
    {label: "Publish date", value: createdAt},
    {label: "Media", value: mediaCounts(unit.downloads)},
    {label: "Author", value: unit.author},
    {label: "Description", value: unit.description},
  ];
  return (
    <div className="flex flex-col">
      <h4 className="header4">Details</h4>

      <MediaViewer downloads={unit.downloads} />

      <h5 className="header5 font-bold text-sapphire uppercase mt-4">Unit</h5>

      <Description items={items} />

      <h5 className="header5 font-bold text-sapphire uppercase mt-4">
        Fetch Info
      </h5>

      <Description items={[{label: "Fetch date", value: fetchedAt}]} />

      {unit.tags.length > 0 && (
        <>
          <h5 className="header5 font-bold text-sapphire uppercase mt-4">
            Tags
          </h5>
          <div className="py-5">
            <QueryTagList tags={unit.tags} />
          </div>
        </>
      )}
    </div>
  );
};

export default DataDetails;
