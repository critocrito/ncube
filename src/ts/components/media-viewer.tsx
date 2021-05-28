import React, {useState} from "react";
import {Swiper as SwiperType} from "swiper";
import {Swiper, SwiperSlide} from "swiper/react";

import {useWorkspaceCtx} from "../lib/context";
import chevronLeft from "../svg/chevron_left.svg";
import chevronRight from "../svg/chevron_right.svg";
import {Download} from "../types";
import VideoPlayer from "./video-player";

interface MediaViewerProps {
  downloads: Download[];
}

interface SlideProps {
  url: string;
  type: string;
}

const MediaSlide = ({url, type}: SlideProps) => {
  switch (type) {
    case "image":
      return <img src={url} alt="Associated media download." />;
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
      const poster = `${url.slice(0, Math.max(0, url.lastIndexOf(".")))}.jpg`;
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
    <div className="relative w-full flex items-center">
      <div className="w-full">
        <Swiper
          slidesPerView={1}
          onSwiper={setSwiper}
          thumbs={{swiper: thumbsSwiper}}
          loop
        >
          {downloads.map(({id_hash: idHash, location, type}) => {
            const url = `http://127.0.0.1:40666/api/workspaces/${slug}/${location}`;

            return (
              <SwiperSlide key={idHash}>
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
