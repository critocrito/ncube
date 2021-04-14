import React, {useEffect, useRef} from "react";

import closeIcon from "../../../resources/public/images/icon_close.svg";
import {isMouseEvent} from "../lib/utils";

interface ModalProps {
  title: string;
  description: string;
  children: JSX.Element;
  onCancel: () => void;
  className?: string;
}

const Modal = ({title, description, onCancel, children}: ModalProps) => {
  // eslint-disable-next-line unicorn/no-null
  const ref = useRef<HTMLDivElement>(null);

  // Collapse the modal when we click outside the menu.
  useEffect(() => {
    const handleClickOutside = (ev: Event) => {
      if (isMouseEvent(ev) && !ref.current?.contains(ev.target as Node)) {
        onCancel();
      }
    };

    document.addEventListener("click", handleClickOutside, true);
    return () => {
      document.removeEventListener("click", handleClickOutside, true);
    };
  }, [onCancel]);

  // Collapse the modal when we press the escape key.
  useEffect(() => {
    const handleEscapeKey = (ev: KeyboardEvent) => {
      const key = ev.key || ev.keyCode;
      if (key === "Escape" || key === "Esc" || key === 27) {
        onCancel();
      }
    };

    document.addEventListener("keydown", handleEscapeKey, false);
    return () => {
      document.removeEventListener("keydown", handleEscapeKey, false);
    };
  }, [onCancel]);

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
        ></div>

        <span
          className="hidden sm:inline-block sm:align-middle sm:h-screen"
          aria-hidden="true"
        >
          &#8203;
        </span>

        <div
          ref={ref}
          className="inline-block align-bottom bg-white rounded-sm px-4 pt-5 pb-4 text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full sm:p-6"
        >
          <div className="hidden sm:block absolute top-0 right-0 pt-4 pr-4">
            <button
              type="button"
              onClick={onCancel}
              aria-label="Close Modal"
              className="bg-white rounded-md text-gray-400 hover:text-gray-500"
            >
              <span className="sr-only">Close</span>
              <img src={closeIcon} className="w-4 h-4" alt="Close modal." />
            </button>
          </div>
          <div className="sm:flex sm:items-start">{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Modal;
