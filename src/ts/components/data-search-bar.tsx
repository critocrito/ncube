import React, {KeyboardEvent} from "react";

import iconSearch from "../../../resources/public/images/icon_search.svg";

interface DataSearchBarProps {
  query: string;
  onSearch: (q: string) => void;
  onChange: (q: string) => void;
  isDisabled?: boolean;
}

const DataSearchBar = ({
  onSearch,
  query,
  onChange,
  isDisabled = false,
}: DataSearchBarProps) => {
  const onEnter = (ev: KeyboardEvent<HTMLInputElement>) => {
    if (ev.key === "Enter") onSearch(query);
  };

  return (
    <div className="mt-1 relative w-96">
      <input
        type="text"
        name="search-bar"
        id="search-bar"
        onChange={(ev) => onChange(ev.target.value)}
        onKeyPress={onEnter}
        className="block w-full pr-10 sm:text-sm border-solitude rounded-md"
      />
      <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
        <button
          type="button"
          className="bg-white"
          aria-label="toggle menu"
          disabled={isDisabled}
          onClick={() => onSearch(query)}
        >
          <img src={iconSearch} className="w-6 h-6" alt="Do a search." />
        </button>
      </div>
    </div>
  );
};

export default DataSearchBar;
