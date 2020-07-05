/* eslint react/jsx-props-no-spreading: off, react/no-array-index-key: off */
import c from "classnames";
import {useCombobox, useMultipleSelection} from "downshift";
import {FormikProps, withFormik} from "formik";
import matchSorter from "match-sorter";
import React, {useState} from "react";

import iconChevronDown from "../../../resources/public/images/icon_chevron_down.svg";
import iconChevronUp from "../../../resources/public/images/icon_chevron_up.svg";
import iconTrash from "../../../resources/public/images/icon_trash.svg";
import Button from "../common/button";
import QueryTag from "../common/query-tag";
import {SourceTag} from "../types";
import * as v from "../validations";

interface MultiSelectProps {
  data: SourceTag[];
  onAdd: (tag: SourceTag) => void;
  onRemove: (tag: SourceTag) => void;
  className?: string;
}

interface AddSourceProps {
  onAdd: (label: string, description?: string) => void;
}

interface AddSourceFormValues {
  label: string;
  description?: string;
}

const AddForm = withFormik<AddSourceProps, AddSourceFormValues>({
  handleSubmit: (_, {resetForm}) => {
    resetForm();
  },
  displayName: "AddTagForm",
  validationSchema: v.sourceTag,
})(
  ({
    values,
    touched,
    errors,
    handleChange,
    handleBlur,
    handleSubmit,
    onAdd,
    isSubmitting,
  }: AddSourceProps & FormikProps<AddSourceFormValues>) => {
    const hasErrorLabel = errors.label && touched.label;
    const hasErrorDescription = errors.description && touched.description;
    const isDisabled = hasErrorLabel || hasErrorDescription || isSubmitting;

    return (
      <div className="flex flex-column">
        <div className="flex justify-between items-center ml2">
          <div className="flex justify-between w-two-thirds">
            <input
              className={c(
                "fb1 pa2 ba",
                hasErrorLabel ? "b--error" : "b--solitude",
              )}
              name="label"
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.label || ""}
              placeholder="Label"
            />

            <input
              className={c(
                "fb1 pa2 ba ml2",
                hasErrorDescription ? "b--error" : "b--solitude",
              )}
              name="description"
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.description || ""}
              placeholder="Description"
            />
          </div>

          <Button
            className="mr2 w-third fr"
            type="button"
            disabled={isDisabled}
            onClick={() => {
              onAdd(
                values.label,
                values.description === "" ? undefined : values.description,
              );
              handleSubmit();
            }}
          >
            Add
          </Button>
        </div>

        <div className="flex justify-between ml2">
          <div className="flex justify-between w-two-thirds">
            <div className="w-50">
              {hasErrorLabel && <div className="error">{errors.label}</div>}
            </div>

            <div className="w-50 ml2">
              {hasErrorDescription && (
                <div className="error">{errors.description}</div>
              )}
            </div>
          </div>

          <div className="w-third" />
        </div>
      </div>
    );
  },
);

const MultiSelect = ({onAdd, onRemove, className, data}: MultiSelectProps) => {
  const [createdData, setCreatedData] = useState<SourceTag[]>([]);
  const [query, setQuery] = useState<string>("");

  const getFilteredItems = (selected: SourceTag[], items: SourceTag[]) => {
    const selection = items.filter((item) => {
      return !selected.includes(item);
    });
    return matchSorter(selection, query, {keys: ["label", "description"]});
  };

  const {
    getSelectedItemProps,
    getDropdownProps,
    addSelectedItem,
    removeSelectedItem,
    selectedItems,
  } = useMultipleSelection<SourceTag>({});

  const {
    isOpen,
    getToggleButtonProps,
    getLabelProps,
    getMenuProps,
    getInputProps,
    getComboboxProps,
    getItemProps,
    highlightedIndex,
  } = useCombobox<SourceTag>({
    inputValue: query,
    defaultHighlightedIndex: 0, // after selection, highlight the first item.
    selectedItem: undefined,
    items: getFilteredItems(selectedItems, data),

    stateReducer: (_state, actionAndChanges) => {
      const {changes, type} = actionAndChanges;
      // eslint-disable-next-line default-case
      switch (type) {
        case useCombobox.stateChangeTypes.InputKeyDownEnter:
        case useCombobox.stateChangeTypes.ItemClick:
          return {
            ...changes,
            isOpen: true, // keep the menu open after selection.
          };
      }
      return changes;
    },

    // type error: https://github.com/downshift-js/downshift/issues/1015
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    onStateChange: ({inputValue, type, selectedItem}) => {
      switch (type) {
        case useCombobox.stateChangeTypes.InputChange:
          setQuery(inputValue || "");
          break;
        case useCombobox.stateChangeTypes.InputKeyDownEnter:
        case useCombobox.stateChangeTypes.ItemClick:
        case useCombobox.stateChangeTypes.InputBlur:
          if (selectedItem) {
            setQuery("");
            addSelectedItem(selectedItem);
          }
          break;
        default:
          break;
      }
    },
  });

  return (
    <div className={c(className)}>
      {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
      <label htmlFor="multi-select" {...getLabelProps()}>
        Choose some elements:
      </label>
      <div>
        <ul className="list pl0 mt2 mb2">
          {createdData.map((selectedItem, index) => {
            return (
              <li
                className="flex justify-between mt1 mb1"
                key={`selected-item-${selectedItem.label}`}
                {...getSelectedItemProps({selectedItem, index})}
              >
                <div className="w-90 flex">
                  <QueryTag
                    className="mr1"
                    label={selectedItem.label}
                    description={selectedItem.description}
                  />

                  <span className="ml2 gray-25">
                    {selectedItem.description}
                  </span>
                </div>

                <button
                  type="button"
                  className="b--none bg-canvas"
                  onClick={() => {
                    onRemove(selectedItem);
                    setCreatedData(
                      createdData.filter(
                        (item) => item.label !== selectedItem.label,
                      ),
                    );
                  }}
                >
                  <img
                    height="25px"
                    width="25px"
                    alt="Remove source tag."
                    src={iconTrash}
                  />
                </button>
              </li>
            );
          })}

          {selectedItems.map((selectedItem, index) => {
            return (
              <li
                className="flex justify-between mt1 mb1"
                key={`selected-item-${selectedItem.label}`}
                {...getSelectedItemProps({selectedItem, index})}
              >
                <div className="w-90 flex">
                  <QueryTag
                    className="mr1"
                    label={selectedItem.label}
                    description={selectedItem.description}
                  />

                  <span className="ml2 gray-25">
                    {selectedItem.description}
                  </span>
                </div>

                <button
                  type="button"
                  className="b--none bg-canvas"
                  onClick={() => {
                    onRemove(selectedItem);
                    removeSelectedItem(selectedItem);
                  }}
                >
                  <img
                    height="25px"
                    width="25px"
                    alt="Remove source tag."
                    src={iconTrash}
                  />
                </button>
              </li>
            );
          })}
        </ul>

        <div className="select flex items-center" {...getComboboxProps()}>
          <input
            id="multi-select"
            name="multi-select"
            type="text"
            value={query}
            {...getInputProps(getDropdownProps({preventKeyAction: isOpen}))}
          />
          <button
            type="button"
            className="b--none bg-white"
            {...getToggleButtonProps()}
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
      </div>

      <div {...getMenuProps()}>
        {isOpen && (
          <div>
            <ul
              className="pl0 mt0 mb0 pl0 ba b--solitude bg-white"
              style={{maxHeight: "180px", overflowY: "auto"}}
            >
              {getFilteredItems(selectedItems, data).map((item, index) => {
                const {onClick, ...itemProps} = getItemProps({
                  item,
                  index,
                });
                return (
                  <li
                    key={`listed-item-${item.label}`}
                    className={c(
                      "mt2 mb2 pa1",
                      highlightedIndex === index ? "bg-fair-pink" : undefined,
                    )}
                    {...itemProps}
                  >
                    <button
                      onClick={(ev) => {
                        onAdd(item);
                        onClick(ev);
                      }}
                      className={c(
                        "b--none flex",
                        highlightedIndex === index ? "bg-fair-pink" : undefined,
                      )}
                    >
                      <QueryTag className="w-30 mr2" label={item.label} />
                      <span className="w-70 gray-25">{item.description}</span>
                    </button>
                  </li>
                );
              })}
            </ul>

            <div className="pl1 pt2 pb2 pr1 ba b--solitude bg-white">
              <AddForm
                onAdd={(label: string, description?: string) => {
                  const item = {label, description};
                  onAdd(item);
                  setCreatedData(createdData.concat([item]));
                }}
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default MultiSelect;
