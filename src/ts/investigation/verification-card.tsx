/* eslint react/jsx-props-no-spreading: off */
import React from "react";
import {Draggable} from "react-beautiful-dnd";
import {EventObject} from "xstate";

import SourceTag from "../common/source-tag";
import {Platform, SegmentUnit} from "../types";

interface VerificationCardProps<
  TContext extends unknown,
  TEvent extends EventObject
> {
  unit: SegmentUnit<TContext, TEvent>;
  index: number;
}

const VerificationCard = <
  TContext extends unknown,
  TEvent extends EventObject
>({
  unit,
  index,
}: VerificationCardProps<TContext, TEvent>) => {
  let platform: Platform;

  switch (true) {
    case unit.source.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case unit.source.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <Draggable draggableId={unit.id.toString()} index={index}>
      {(provided, _snapshot) => (
        <div
          onClick={() => {}}
          onKeyPress={() => {}}
          tabIndex={0}
          role="button"
          className="h4 ba b--solitude ma2 flex flex-column justify-between bg-white"
          ref={provided.innerRef}
          {...provided.draggableProps}
          {...provided.dragHandleProps}
        >
          <div className="h-70 pa2">{unit.title}</div>

          <div className="flex items-center justify-between bg-canvas h-30 pa2">
            <SourceTag kind={platform} className="br b--solitude mv2" />
            <div>&nbsp;</div>
          </div>
        </div>
      )}
    </Draggable>
  );
};

export default VerificationCard;
