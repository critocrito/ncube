/* eslint react/jsx-props-no-spreading: off */
import c from "clsx";
import {csvFormat} from "d3-dsv";
import React, {useCallback, useState} from "react";
import {DragDropContext, Draggable, Droppable} from "react-beautiful-dnd";
import {Machine} from "xstate";

import {listUnitsByIds} from "../lib/http";
import {voidFn} from "../lib/utils";
import {Methodology, Segment, SegmentUnit, Workspace} from "../types";
import Placeholder from "./placeholder";
import VerificationCard from "./verification-card";
import VerificationColumn from "./verification-column";

interface VerificationCanvasProps {
  workspace: Workspace;
  segment: Segment;
  methodology: Methodology;
  units: Map<string, SegmentUnit[]>;
  onDetails?: (unit: SegmentUnit) => void;
  onMoveEnd?: (
    unit: number,
    from: string,
    to: string,
    position: number,
  ) => void;
}

const VerificationCanvas = ({
  workspace: {slug: workspaceSlug},
  segment: {slug: segmentSlug},
  methodology,
  units,
  onDetails = voidFn,
  onMoveEnd = voidFn,
}: VerificationCanvasProps) => {
  const [allowedColumns, setAllowedColumns] = useState<string[]>([]);

  const onDragStart = useCallback(
    (event) => {
      if (!methodology) return;

      const {
        draggableId,
        source: {droppableId: sourceId},
      } = event;
      if (!sourceId) return;

      const sourceUnits = units.get(sourceId);
      if (!sourceUnits) return;

      const unit = sourceUnits.find(({id}) => id.toString() === draggableId);
      if (!unit) return;

      const machine = Machine(methodology.process);
      const currentState = machine.resolveState(unit.state);
      const nextStates = currentState.nextEvents;

      setAllowedColumns(
        nextStates.map((ev) => ev.replace(/^TO_/, "").toLowerCase()),
      );
    },
    [units, methodology],
  );

  const onDragEnd = useCallback(
    (event) => {
      // We remove the column highlighting in any case.
      setAllowedColumns([]);

      const {draggableId} = event;
      const sourceId = event?.source?.droppableId;
      const destinationId = event?.destination?.droppableId;
      const index = event?.destination?.index;

      onMoveEnd(
        Number.parseInt(draggableId, 10),
        sourceId,
        destinationId,
        index,
      );
    },
    [onMoveEnd],
  );

  if (!methodology) return <Placeholder />;

  return (
    <div className="overflow-x-auto">
      <DragDropContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
        <div>
          <div className="flex space-x-6">
            {[...units.keys()].map((name, i) => {
              const isDroppable =
                allowedColumns.length === 0 || allowedColumns.includes(name);
              const data = units.get(name) || [];

              return (
                <Droppable key={name} droppableId={name}>
                  {(provided, snapshot) => (
                    <div ref={provided.innerRef} {...provided.droppableProps}>
                      <VerificationColumn
                        name={name}
                        cntUnits={data.length}
                        isHighlighted={snapshot.isDraggingOver}
                        isDroppable={isDroppable}
                        onDownload={async () => {
                          // FIXME: I know this is messy code. I want to
                          // revisit the CSV generation and push it into the
                          // backend. I believe this will be much more efficient
                          // than doing it in the UI. I also need to extend the CSV
                          // download with downloads, tags and annotations.
                          const ids = data.map(({id}) => id);
                          const csvUnits = await listUnitsByIds(
                            workspaceSlug,
                            ids,
                          );

                          const filename = `${segmentSlug}-${name}.csv`;
                          const csv = csvFormat(
                            csvUnits.map(
                              ({
                                id,
                                id_hash,
                                source,
                                unit_id,
                                body,
                                href,
                                author,
                                title,
                                description,
                                created_at,
                                fetched_at,
                              }) => ({
                                id,
                                id_hash,
                                source,
                                unit_id,
                                body,
                                href,
                                author,
                                title,
                                description,
                                created_at,
                                fetched_at,
                              }),
                            ),
                          );
                          const blob = new Blob([csv], {type: "text/csv"});
                          const url = URL.createObjectURL(blob);
                          const a = document.createElement("a");
                          a.href = url;
                          a.download = filename || "download.csv";
                          const clickHandler = () => {
                            setTimeout(() => {
                              URL.revokeObjectURL(url);
                              a.removeEventListener("click", clickHandler);
                            }, 150);
                          };
                          a.addEventListener("click", clickHandler, false);
                          a.click();
                        }}
                      >
                        <div className="space-y-4">
                          {data.map((unit, index) => (
                            <Draggable
                              key={unit.id}
                              draggableId={unit.id.toString()}
                              index={index}
                            >
                              {(unitProvided, _snapshot) => (
                                <div
                                  onClick={() => onDetails(unit)}
                                  onKeyPress={() => onDetails(unit)}
                                  tabIndex={0}
                                  role="button"
                                  ref={unitProvided.innerRef}
                                  {...unitProvided.draggableProps}
                                  {...unitProvided.dragHandleProps}
                                >
                                  <VerificationCard unit={unit} />
                                </div>
                              )}
                            </Draggable>
                          ))}
                        </div>
                      </VerificationColumn>
                      {provided.placeholder}
                    </div>
                  )}
                </Droppable>
              );
            })}
          </div>
        </div>
      </DragDropContext>
    </div>
  );
};

export default VerificationCanvas;
