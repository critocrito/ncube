import "video.js/dist/video-js.css";

import React, {useEffect, useState} from "react";
import videojs, {VideoJsPlayer} from "video.js";

interface VideoPlayerProps {
  src: string;
}

const VideoPlayer = ({src}: VideoPlayerProps) => {
  const [player, setPlayer] = useState<VideoJsPlayer>();
  const videoNode = React.createRef<HTMLVideoElement>();
  const poster = `${src.slice(0, Math.max(0, src.lastIndexOf(".")))}.jpg`;

  useEffect(() => {
    if (videoNode.current)
      setPlayer(
        videojs(videoNode.current, {
          controls: true,
          fluid: true,
          sources: [{src}],
        }),
      );

    return () => {
      if (player) player.dispose();
    };
  }, [videoNode, player, src]);

  return (
    <div data-vjs-player>
      {/* eslint-disable-next-line jsx-a11y/media-has-caption */}
      <video poster={poster} ref={videoNode} className="video-js z-10" />
    </div>
  );
};

export default VideoPlayer;
