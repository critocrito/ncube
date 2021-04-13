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
    <div className="search-bar flex items-center w-90">
      <input
        className="w-90"
        id="search-bar"
        name="search-bar"
        type="text"
        value={query}
        disabled={isDisabled}
        onChange={(ev) => onChange(ev.target.value)}
        onKeyPress={onEnter}
      />

      <button
        type="button"
        className="b--none bg-white"
        aria-label="toggle menu"
        disabled={isDisabled}
        onClick={() => onSearch(query)}
      >
        <img src={iconSearch} width="30px" height="30px" alt="Do a search." />
      </button>
    </div>
  );
};

export default DataSearchBar;
