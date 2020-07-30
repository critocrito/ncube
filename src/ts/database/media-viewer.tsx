import React from "react";

import VideoPlayer from "../common/video-player";
import {useWorkspaceCtx} from "../context";
import {Download} from "../types";

interface MediaViewerProps {
  downloads: Download[];
}

const MediaViewer = ({downloads}: MediaViewerProps) => {
  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  if (downloads.length === 0) return <div />;

  const views = downloads.map(({type, location}) => {
    const url = `http://127.0.0.1:40666/api/workspaces/${slug}/${location}`;

    switch (type) {
      case "image":
        return <img src={url} alt="Associated media download." />;
      case "video":
        return <VideoPlayer src={url} />;
      default:
        return <div />;
    }
  });

  return <div className="flex flex-column">{views}</div>;
};

export default MediaViewer;
