import c from "classnames";
import React from "react";

import closeIcon from "../../../resources/public/images/icon_close.svg";

interface ModalProps {
  title: string;
  description: string;
  children: JSX.Element;
  onCancel: () => void;
  className?: string;
}

const Modal = ({
  title,
  description,
  onCancel,
  children,
  className,
}: ModalProps) => {
  return (
    <>
      <div
        role="dialog"
        aria-labelledby={title}
        aria-describedby={description}
        className={c(
          "absolute top-2 ml-auto h-40 w-50 bg-canvas z-9999",
          className,
        )}
      >
        <div className="relative">
          <button
            aria-label="Close Modal"
            className="absolute bg-canvas b--none top-0 right-0 mt2"
            onClick={onCancel}
          >
            <img
              src={closeIcon}
              height="15px"
              width="15px"
              alt="Close modal."
            />
          </button>
          <div className="pa4">{children}</div>
        </div>
      </div>
      <div className=" absolute bg-black o-50 w-100 h-100 fixed z-999 top-0 left-0 flex items-center justify-around" />
    </>
  );
};

export default Modal;
