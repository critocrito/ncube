import c from "clsx";
import React from "react";

import bookmark from "../svg/bookmark.svg";
import {SelectOption} from "../types";
import SelectDropdown from "./select-dropdown";

interface TabsProps {
  items: Array<{label: string; value: string}>;
  selected: {label: string; value: string};
  onClick: (item: {label: string; value: string}) => void;
}

const Tabs = ({items, selected, onClick}: TabsProps) => {
  return (
    <div>
      <div className="sm:hidden">
        {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
        <label htmlFor="input-tab-selector" className="sr-only">
          Select a tab
        </label>
        <SelectDropdown<SelectOption>
          id="tab-selector"
          options={items}
          defaultValue={selected}
          onSelect={(option) => {
            if (!option) return;
            onClick(option);
          }}
          className="w-full rounded mb-2 cursor-pointer border border-solitude"
        />
      </div>

      <div className="hidden sm:block">
        <nav className="-mb-px flex" aria-label="Tabs">
          {items.map(({label, value}) => {
            const isSelected = selected.value === value;
            const className = c(
              "justify-around rounded-t-xl w-48 text-sapphire group inline-flex items-center py-4 px-1 font-bold header5",
              {
                "bg-white": isSelected,
              },
            );
            return (
              <button
                key={value}
                className={className}
                onClick={() => onClick({label, value})}
              >
                {isSelected ? (
                  <div className="flex items-center">
                    <img
                      className="w-5 h-5"
                      src={bookmark}
                      alt="Selected tab."
                    />
                    <span className="ml-3">{label}</span>
                  </div>
                ) : (
                  <span>{label}</span>
                )}
              </button>
            );
          })}
        </nav>
      </div>
    </div>
  );
};

export default Tabs;
