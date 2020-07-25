import React, {KeyboardEvent} from "react";

import iconSearch from "../../../resources/public/images/icon_search.svg";

interface SearchBarProps {
  query: string;
  onSearch: (q: string) => void;
  onChange: (q: string) => void;
}

const SearchBar = ({onSearch, query, onChange}: SearchBarProps) => {
  const onEnter = (ev: KeyboardEvent<HTMLInputElement>) => {
    if (ev.key === "Enter") onSearch(query);
  };

  return (
    <div className="select flex items-center">
      <input
        id="multi-select"
        name="multi-select"
        type="text"
        value={query}
        onChange={(ev) => onChange(ev.target.value)}
        onKeyPress={onEnter}
      />

      <button
        type="button"
        className="b--none bg-white"
        aria-label="toggle menu"
        onClick={() => onSearch(query)}
      >
        <img src={iconSearch} width="14px" height="14px" alt="Do a search." />
      </button>
    </div>
  );
};

export default SearchBar;
