import React, {KeyboardEvent} from "react";

import iconHelp from "../../../../resources/public/images/icon_help.svg";
import iconSearch from "../../../../resources/public/images/icon_search.svg";

interface SearchBarProps {
  query: string;
  onSearch: (q: string) => void;
  onChange: (q: string) => void;
  onHelp?: () => void;
}

const SearchBar = ({onSearch, query, onChange, onHelp}: SearchBarProps) => {
  const onEnter = (ev: KeyboardEvent<HTMLInputElement>) => {
    if (ev.key === "Enter") onSearch(query);
  };

  return (
    <div className="flex items-center">
      <div className="search-bar flex items-center w-90">
        <input
          className="w-90"
          id="search-bar"
          name="search-bar"
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
          <img src={iconSearch} width="30px" height="30px" alt="Do a search." />
        </button>
      </div>

      {onHelp && (
        <button
          className="b--none bg-canvas ml2"
          aria-label="toggle menu"
          onClick={onHelp}
        >
          <img
            src={iconHelp}
            width="14px"
            height="14px"
            alt="Open search help."
          />
        </button>
      )}
    </div>
  );
};

export default SearchBar;
