import c from "classnames";
import React from "react";
import {TableInstance} from "react-table";

import Button from "../common/button";
// import addIcon from "../../../resources/public/images/icon_add.svg";

interface ActionBarProps<T extends {id: number}> {
  instance: TableInstance<T>;
  handleSelected: (ids: string[]) => void;
  onCreate?: () => void;
  className?: string;
}

const ActionBar = <T extends {id: number}>({
  instance,
  onCreate,
  handleSelected,
  className,
}: ActionBarProps<T>) => {
  const {state} = instance;

  const classes = c("flex justify-between items-center", className);
  const countSelectedItems = Object.keys(state.selectedRowIds).length;

  const createButton = onCreate ? (
    <Button className="flex items-center" onClick={onCreate} kind="secondary">
      {/* <img
            className="bg-white mr2"
            height="15px"
            width="15px"
            src={addIcon}
            alt="Add."
            />
          */}{" "}
      <span>Add New</span>
    </Button>
  ) : (
    ""
  );

  const selectButton = (
    <div className="flex items-center">
      {countSelectedItems === 0 ? (
        ""
      ) : (
        <span className="sapphire text-medium b mr3">
          {countSelectedItems} selected
        </span>
      )}
      <Button
        onClick={() => handleSelected(Object.keys(state.selectedRowIds))}
        disabled={countSelectedItems === 0}
        size="large"
      >
        Send to process
      </Button>
    </div>
  );

  return (
    <div className={classes}>
      {createButton}
      {selectButton}
    </div>
  );
};

export default ActionBar;
