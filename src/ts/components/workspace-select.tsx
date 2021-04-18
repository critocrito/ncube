import c from "clsx";
import React from "react";
import {SingleValueProps} from "react-select";

import {SelectOption, Workspace} from "../types";
import SelectDropdown from "./select-dropdown";

interface WorkspaceSelectProps {
  workspaces: Workspace[];
  onChange: (w: Workspace) => void;
  selectedWorkspace: Workspace;
  className?: string;
}

export const SingleValue = ({
  innerProps,
  data,
}: SingleValueProps<SelectOption>) => {
  return (
    <div className="font-bold text-white" {...innerProps}>
      {data.label}
    </div>
  );
};

const WorkspaceSelect = ({
  workspaces,
  selectedWorkspace,
  onChange,
  className,
}: WorkspaceSelectProps) => {
  const options = workspaces.map(({name: label, slug: value}) => ({
    label,
    value,
  }));

  return (
    <SelectDropdown<SelectOption>
      id="workspace-selector"
      options={options}
      defaultValue={{
        label: selectedWorkspace?.name,
        value: selectedWorkspace?.slug,
      }}
      onSelect={(option) => {
        if (!option) return;
        const workspace = workspaces.find(({slug}) => slug === option.value);
        if (workspace) onChange(workspace);
      }}
      LocalSingleValue={SingleValue}
      className={c("w-full rounded mb-2", className)}
    />
  );
};

export default WorkspaceSelect;
