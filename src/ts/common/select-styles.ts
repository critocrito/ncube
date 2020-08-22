import {CSSProperties} from "react";
import {StylesConfig} from "react-select";

interface StateProps {
  [key: string]: unknown;
}

// It seems a bit weird that I have to define this pseudo element. But
// Typescript is complaining otherwise.
interface HoverCSSProperties extends CSSProperties {
  ":hover"?: CSSProperties;
}

const option = (
  provided: HoverCSSProperties,
  state: StateProps,
): HoverCSSProperties =>
  ({
    ...provided,
    borderColor: "#dfe2ec",
    color: state.isSelected ? "#0a2463" : "#0a2463",
    opacity: state.isDisabled ? 0.5 : 1,
    backgroundColor: state.isSelected ? "#f8ece8" : "white,",
    ":hover": {
      ...provided[":hover"],
      backgroundColor: "#f8ece8",
    },
  } as HoverCSSProperties);

export default {
  control: (provided) => ({
    ...provided,
    borderColor: "#dfe2ec",
  }),
  option,
} as StylesConfig;
