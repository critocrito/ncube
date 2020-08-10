/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import {useSelect} from "downshift";
import {useField} from "formik";
import React from "react";

import iconChevronDown from "../../../resources/public/images/icon_chevron_down.svg";
import iconChevronUp from "../../../resources/public/images/icon_chevron_up.svg";
import LabeledSourceTag from "../common/labeled-source-tag";
import {SourceType} from "../types";

interface SourceTypeSelectProps {
  initial?: string;
}

const items: SourceType[] = [
  "youtube_video",
  "youtube_channel",
  "twitter_tweet",
  "twitter_user",
  "http_url",
];

const SourceTypeSelect = ({initial}: SourceTypeSelectProps) => {
  const {
    isOpen,
    selectedItem,
    getToggleButtonProps,
    getLabelProps,
    getMenuProps,
    highlightedIndex,
    getItemProps,
  } = useSelect({items, initialSelectedItem: initial});

  const [, meta, {setValue, setTouched}] = useField({name: "type"});

  const {touched, error} = meta;

  const hasError = touched && error;
  const {onClick: onToggle, ...toggleButtonProps} = getToggleButtonProps();

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <div className="mb1" {...getLabelProps()}>
        Detected source type:
      </div>

      <div
        className={c(
          "flex items-center justify-between pa2 ba bg-white",
          hasError ? "b--error" : "b--solitude",
        )}
      >
        {selectedItem ? <LabeledSourceTag label={selectedItem} /> : <div />}

        <button
          type="button"
          className="b--none bg-white"
          onClick={(ev) => {
            // This little weirdness makes sure that the errors are displayed
            // only once the drop down was opened and closed again.
            if (isOpen) setTouched(true);
            onToggle(ev);
          }}
          {...toggleButtonProps}
          aria-label="toggle menu"
        >
          <img
            src={isOpen ? iconChevronUp : iconChevronDown}
            width="14px"
            height="14px"
            alt="Chevron up/down icon."
          />
        </button>
      </div>

      <div {...getMenuProps()}>
        {isOpen && (
          <ul
            className="pl0 mt0 mb0 pl0 ba b--solitude bg-white"
            style={{maxHeight: "225px", overflowY: "auto"}}
          >
            {items.map((item, index) => {
              const {onClick, ...itemProps} = getItemProps({
                item,
                index,
              });

              return (
                <li
                  key={`source-tag-item-${item}`}
                  className={c(
                    "mt2 mb2 pa1",
                    highlightedIndex === index ? "bg-fair-pink" : undefined,
                  )}
                  {...itemProps}
                >
                  <div
                    onClick={(ev) => {
                      setValue(item);
                      onClick(ev);
                    }}
                    onKeyPress={(ev) => {
                      setValue(item);
                      onClick(ev);
                    }}
                    tabIndex={0}
                    role="button"
                    className={c(
                      "b--none bg-white",
                      highlightedIndex === index ? "bg-fair-pink" : undefined,
                    )}
                  >
                    <LabeledSourceTag label={item} />
                  </div>
                </li>
              );
            })}
          </ul>
        )}
      </div>

      {hasError ? <div className="error">{meta.error}</div> : undefined}
    </div>
  );
};

export default SourceTypeSelect;
