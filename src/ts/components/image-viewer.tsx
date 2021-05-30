import Drift from "drift-zoom";
import React, {useEffect} from "react";

interface ImageViewerProps {
  src: string;
}

const ImageViewer = ({src}: ImageViewerProps) => {
  const ref = React.useRef<HTMLImageElement>(null);
  const paneRef = React.useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (ref.current) {
      // The `|| undefined` is required to avoid the following error;
      // error TS2322: Type 'HTMLDivElement | null' is not assignable to type
      // 'HTMLElement | undefined'. Type 'null' is not assignable to type
      // 'HTMLElement | undefined'.
      // eslint-disable-next-line no-new
      new Drift(ref.current, {paneContainer: paneRef.current || undefined});
    }
  }, []);

  return (
    <>
      <img
        ref={ref}
        className="z-10"
        src={src}
        data-zoom={src}
        alt="Associated media download."
      />
      <div ref={paneRef} />
    </>
  );
};

export default ImageViewer;
