import "video.js/dist/video-js.css";
import React, {useState, useEffect} from "react";
import videojs from "video.js";

interface VideoPlayerProps {
  src: string;
}

const VideoPlayer = ({src}: VideoPlayerProps) => {
  const [player, setPlayer] = useState();
  const videoNode = React.createRef<HTMLVideoElement>();
  const poster = src.substr(0, src.lastIndexOf(".")) + ".jpg";

  useEffect(() => {
    if (videoNode.current)
      setPlayer(
        videojs(videoNode.current, {controls: true, fluid: true, sources: src}),
      );

    () => {
      if (player) player.dispose();
    };
  }, [videoNode]);

  return (
    <div>
      <div data-vjs-player>
        {/* eslint-disable-next-line jsx-a11y/media-has-caption */}
        <video poster={poster} ref={videoNode} className="video-js" />
      </div>
    </div>
  );
};

export default VideoPlayer;
