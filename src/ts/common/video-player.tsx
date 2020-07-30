import React from "react";
import videojs from "video.js";

interface VideoPlayerProps {
  src: string;
}

export default class VideoPlayer extends React.Component<VideoPlayerProps> {
  player: videojs.Player | undefined = undefined;

  private videoNode = React.createRef<HTMLVideoElement>();

  componentDidMount() {
    // instantiate Video.js
    this.player = videojs(this.videoNode);
  }

  // destroy player on unmount
  componentWillUnmount() {
    if (this.player) {
      this.player.dispose();
    }
  }

  // wrap the player in a div with a `data-vjs-player` attribute
  // so videojs won't create additional wrapper in the DOM
  // see https://github.com/videojs/video.js/pull/3856
  render() {
    const {src} = this.props;

    return (
      <div>
        <div data-vjs-player>
          {/* eslint-disable-next-line jsx-a11y/media-has-caption */}
          <video src={src} ref={this.videoNode} className="video-js" />
        </div>
      </div>
    );
  }
}
