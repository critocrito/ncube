/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import {useSelect} from "downshift";
import {useField} from "formik";
import React from "react";

import iconChevronDown from "../../../resources/public/images/icon_chevron_down.svg";
import iconChevronUp from "../../../resources/public/images/icon_chevron_up.svg";
import {Investigation} from "../types";

interface InvestigationSelectProps {
  name: string;
  investigations: Investigation[];
}

const InvestigationSelectItem = ({item}: {item: Investigation}) => {
  return (
    <div>
      {item.title}
      <span className="w3">&nbsp;</span>
      <span className="text-md solitude">{item.description}</span>
    </div>
  );
};

const InvestigationSelect = ({
  investigations,
  name,
}: InvestigationSelectProps) => {
  const {
    isOpen,
    selectedItem,
    getToggleButtonProps,
    getLabelProps,
    getMenuProps,
    highlightedIndex,
    getItemProps,
  } = useSelect({items: investigations});

  const [, meta, {setValue, setTouched}] = useField({name});

  const {touched, error} = meta;

  const hasError = touched && error;
  const {onClick: onToggle, ...toggleButtonProps} = getToggleButtonProps();

  return (
    <div className="flex flex-column fb1 mt3 mb2">
      <div className="mb1" {...getLabelProps()}>
        Choose a investigation:
      </div>

      <div
        className={c(
          "flex items-center justify-between pa2 ba bg-white",
          hasError ? "b--error" : "b--solitude",
        )}
      >
        {selectedItem ? (
          <InvestigationSelectItem item={selectedItem} />
        ) : (
          <div />
        )}

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
            {investigations.map((item, index) => {
              const {onClick, ...itemProps} = getItemProps({
                item,
                index,
              });

              return (
                <li
                  key={`source-tag-item-${item.slug}`}
                  className={c(
                    "mt2 mb2 pa1",
                    highlightedIndex === index ? "bg-fair-pink" : undefined,
                  )}
                  {...itemProps}
                >
                  <div
                    onClick={(ev) => {
                      setValue(item.slug);
                      onClick(ev);
                    }}
                    onKeyPress={(ev) => {
                      setValue(item.slug);
                      onClick(ev);
                    }}
                    tabIndex={0}
                    role="button"
                    className={c(
                      "b--none bg-white",
                      highlightedIndex === index ? "bg-fair-pink" : undefined,
                    )}
                  >
                    <InvestigationSelectItem item={item} />
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

export default InvestigationSelect;
