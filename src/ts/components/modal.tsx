import c from "clsx";
import React, {useEffect} from "react";

import closeIcon from "../../../resources/public/images/icon_close.svg";
import {useOnOutsideClick} from "../lib/hooks";

interface ModalProps {
  title: string;
  description: string;
  children: JSX.Element;
  onCancel: () => void;
  size?: "normal" | "large";
}

const Modal = ({
  title,
  description,
  onCancel,
  children,
  size = "normal",
}: ModalProps) => {
  const ref = useOnOutsideClick<HTMLDivElement>(onCancel);

  // Disable background scrolling.
  useEffect(() => {
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = "unset";
    };
  }, []);

  return (
    <div
      className="fixed z-10 inset-0 overflow-y-auto"
      aria-labelledby={title}
      aria-describedby={description}
      role="dialog"
      aria-modal="true"
    >
      <div className="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div
          className="fixed inset-0 bg-gray-light bg-opacity-60 transition-opacity"
          aria-hidden="true"
        />

        <span
          className="hidden sm:inline-block sm:align-middle sm:h-screen"
          aria-hidden="true"
        >
          &#8203;
        </span>

        <div
          ref={ref}
          className={c(
            "inline-block align-bottom bg-white rounded-sm px-4 pt-5 pb-4 text-left shadow-xl transform transition-all",
            {
              "sm:my-8 sm:align-middle sm:p-6 sm:max-w-lg sm:w-full":
                size === "normal",
              "md:my-8 md:align-middle md:p-6 md:max-w-4xl md:w-full":
                size === "large",
            },
          )}
        >
          <div className="absolute top-0 right-0 pt-4 pr-4">
            <button
              type="button"
              onClick={onCancel}
              aria-label="Close Modal"
              className="bg-white rounded-md text-sapphire dim"
            >
              <span className="sr-only">Close</span>
              <img src={closeIcon} className="w-4 h-4" alt="Close modal." />
            </button>
          </div>
          <div>{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Modal;
