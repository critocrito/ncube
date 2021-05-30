import React, {useState} from "react";
import {Swiper as SwiperType} from "swiper";
import {Swiper, SwiperSlide} from "swiper/react";

import {useWorkspaceCtx} from "../lib/context";
import {downloadAsFile} from "../lib/utils";
import chevronLeft from "../svg/chevron_left.svg";
import chevronRight from "../svg/chevron_right.svg";
import {Download} from "../types";
import ButtonDownload from "./button-download";
import ImageViewer from "./image-viewer";
import VideoPlayer from "./video-player";

interface MediaViewerProps {
  downloads: Download[];
}

interface SlideProps {
  url: string;
  type: string;
}

const posterForVideo = (url: string) =>
  `${url.slice(0, Math.max(0, url.lastIndexOf(".")))}.jpg`;

const downloadMedia = async (url: string): Promise<void> => {
  // By default fetch uses CORS mode. But when server response doesn't contain
  // 'Access-Control-Allow-Origin' header. It skips response body. Ironically,
  // you have to set mode as 'no-cors' to request opaque resources. Opaque
  // responses can't be accessed with JavaScript but the response can still be
  // served or cached by a service worker.
  const resp = await fetch(url, {mode: "no-cors"});
  const blob = await resp.blob();

  const filename = url.slice(Math.max(0, url.lastIndexOf("/") + 1));
  const mimeType =
    resp.headers.get("Content-Type") || "application/octet-stream";

  downloadAsFile(mimeType, filename, blob);
};

const MediaSlide = ({url, type}: SlideProps) => {
  switch (type) {
    case "image":
      return <ImageViewer src={url} />;
    case "video":
      return <VideoPlayer src={url} />;
    default:
      return <div />;
  }
};

const ThumbSlide = ({url, type}: SlideProps) => {
  switch (type) {
    case "image":
      return (
        <img
          className="cursor-pointer z-10"
          src={url}
          alt="Associated media download."
        />
      );
    case "video": {
      const poster = posterForVideo(url);
      return (
        <img className="cursor-pointer z-10" src={poster} alt="Video poster." />
      );
    }
    default:
      return <div />;
  }
};

const MediaViewer = ({downloads}: MediaViewerProps) => {
  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();
  const [swiper, setSwiper] = useState<SwiperType>();
  const [thumbsSwiper, setThumbsSwiper] = useState<SwiperType>();

  const handlePrevSlide = () => {
    if (swiper) swiper.slidePrev();
  };

  const handleNextSlide = () => {
    if (swiper) swiper.slideNext();
  };

  if (downloads.length === 0) return <div />;

  return (
    <div className="relative w-full flex items-center py-5">
      <div className="w-full space-y-2">
        <Swiper
          slidesPerView={1}
          onSwiper={setSwiper}
          thumbs={{swiper: thumbsSwiper}}
          loop
        >
          {downloads.map(({id_hash: idHash, location, type}) => {
            const url = `http://127.0.0.1:40666/api/workspaces/${slug}/${location}`;
            const handleClick = async () => downloadMedia(url);

            return (
              <SwiperSlide key={idHash}>
                <ButtonDownload
                  className="ml-auto"
                  onClick={handleClick}
                  label="Download Media"
                />
                <MediaSlide key={idHash} url={url} type={type} />
              </SwiperSlide>
            );
          })}
        </Swiper>

        <Swiper
          slidesPerView={3}
          onSwiper={setThumbsSwiper}
          spaceBetween={10}
          watchSlidesVisibility
          watchSlidesProgress
        >
          {downloads.map(({id_hash: idHash, location, type}) => {
            const url = `http://127.0.0.1:40666/api/workspaces/${slug}/${location}`;

            return (
              <SwiperSlide key={idHash}>
                <ThumbSlide key={idHash} url={url} type={type} />
              </SwiperSlide>
            );
          })}
        </Swiper>
      </div>

      <button
        className="slider-button cursor-pointer absolute z-10 -ml-5"
        onClick={handlePrevSlide}
      >
        <img src={chevronLeft} alt="slide left" className="h-5 w-5" />
      </button>

      <button
        className="slider-button cursor-pointer absolute -right-5 z-10"
        onClick={handleNextSlide}
      >
        <img src={chevronRight} alt="slide right" className="h-5 w-5" />
      </button>
    </div>
  );
};

export default MediaViewer;
