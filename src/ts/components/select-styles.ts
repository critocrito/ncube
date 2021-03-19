import {StylesConfig} from "react-select";

type OptionType = {
  label: string;
  value: string;
};

const selectStyle: StylesConfig<OptionType, false> = {
  control: (provided) => ({
    ...provided,
    borderColor: "#dfe2ec",
  }),

  option: (provided, state) => ({
    ...provided,
    borderColor: "#dfe2ec",
    color: state.isSelected ? "#0a2463" : "#0a2463",
    opacity: state.isDisabled ? 0.5 : 1,
    backgroundColor: state.isSelected ? "#f8ece8" : "white,",
    ":hover": {
      ...provided[":hover"],
      backgroundColor: "#f8ece8",
    },
  }),
};

export default selectStyle;
