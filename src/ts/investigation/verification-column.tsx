/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import React from "react";
import {Droppable} from "react-beautiful-dnd";
import {EventObject} from "xstate";

import {SegmentUnit} from "../types";
import VerificationCard from "./verification-card";

interface VerificationColumnProps<
  TContext extends unknown,
  TEvent extends EventObject
> {
  name: string;
  units: SegmentUnit<TContext, TEvent>[];
  isHighlighted?: boolean;
  isDroppable?: boolean;
  className?: string;
}

const VerificationColumn = <
  TContext extends unknown,
  TEvent extends EventObject
>({
  name,
  className,
  units,
  isHighlighted = false,
  isDroppable = true,
}: VerificationColumnProps<TContext, TEvent>) => {
  const isRequiredColumn = [
    "incoming_data",
    "discarded_data",
    "verified_data",
  ].includes(name);

  return (
    <div
      style={{width: "20rem"}}
      className={c(
        "ba b--solitude flex flex-column",
        className,
        isRequiredColumn ? "bg-canvas" : "bg-white",
        isDroppable ? undefined : "o-40",
        isHighlighted && isDroppable ? "shadow-4" : undefined,
      )}
    >
      <div className="bb b--solitude h3 ttu center w-100 pa2 mv2 flex items-center">
        <span className="text-medium b sapphire">{name}</span>
      </div>

      <div className="pa2 mv2">ALL: {units.length}</div>

      <div className="pa2 h-100">
        <Droppable droppableId={name}>
          {(provided, _snapshot) => (
            <div
              ref={provided.innerRef}
              {...provided.droppableProps}
              className="h-100"
            >
              {units.map((unit, index) => (
                <VerificationCard key={unit.id} unit={unit} index={index} />
              ))}
              {provided.placeholder}
            </div>
          )}
        </Droppable>
      </div>
    </div>
  );
};

export default VerificationColumn;
