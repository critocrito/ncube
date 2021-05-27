import "react-responsive-carousel/lib/styles/carousel.min.css";

import React from "react";
import {Carousel} from "react-responsive-carousel";

import VideoPlayer from "../components/video-player";
import {useWorkspaceCtx} from "../lib/context";
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

  const renderThumb = (children) =>
    children.map(({props: {src}}) => {
      let poster = src.substr(0, src.lastIndexOf(".")) + ".jpg";
      return <img src={poster} />;
    });

  const views = downloads.map(({id_hash: idHash, type, location}) => {
    const url = `http://127.0.0.1:40666/api/workspaces/${slug}/${location}`;

    switch (type) {
      case "image":
        return <img key={idHash} src={url} alt="Associated media download." />;
      case "video":
        return <VideoPlayer key={idHash} src={url} />;
      default:
        return <div key={idHash} />;
    }
  });

  return (
    <Carousel dynamicHeight showStatus={false} renderThumbs={renderThumb}>
      {views}
    </Carousel>
  );
};

export default MediaViewer;
