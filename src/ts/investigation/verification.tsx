/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import {csvFormat} from "d3-dsv";
import React, {useCallback, useEffect, useState} from "react";
import {DragDropContext, Draggable, Droppable} from "react-beautiful-dnd";
import {EventObject, Machine} from "xstate";

import {
  listSegmentUnitsByState,
  listUnitsByIds,
  showMethodology,
  updateUnitState,
} from "../http";
import {
  Investigation,
  Methodology,
  MethodologySchema,
  Segment,
  SegmentUnit,
  Workspace,
} from "../types";
import VerificationCard from "./verification-card";
import VerificationColumn from "./verification-column";

interface VerificationProps<
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
> {
  workspace: Workspace;
  investigation: Investigation;
  segment: Segment;
  onDetails: (unit: SegmentUnit<TContext, TEvent>) => void;
}

type VerificationUnits<
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
> = Map<string, SegmentUnit<TContext, TEvent>[]>;

const Verification = <
  TContext extends Record<string, unknown>,
  TStateSchema extends MethodologySchema,
  TEvent extends EventObject
>({
  workspace: {slug: workspaceSlug},
  investigation: {methodology: methodologySlug, slug: investigationSlug},
  segment: {slug: segmentSlug},
  onDetails,
}: VerificationProps<TContext, TEvent>) => {
  const [methodology, setMethodology] = useState<
    Methodology<TContext, TStateSchema, TEvent>
  >();
  const [units, setUnits] = useState<VerificationUnits<TContext, TEvent>>(
    new Map(),
  );
  const [allowedColumns, setAllowedColumns] = useState<string[]>([]);

  useEffect(() => {
    const f = async () => {
      const data: Methodology<
        TContext,
        TStateSchema,
        TEvent
      > = await showMethodology(workspaceSlug, methodologySlug);

      // Make the type checker happy.
      if (!data.process || !data.process.states) return;

      const {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        incoming_data: incomingData,
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        discarded_data: discardedData,
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        verified_data: verifiedData,
        ...states
      } = data.process.states;

      const verificationColumns = [
        "incoming_data",
        ...Object.keys(states),
        "discarded_data",
        "verified_data",
      ];

      const fetchedUnits = await Promise.all(
        verificationColumns.map((name) =>
          listSegmentUnitsByState(
            workspaceSlug,
            investigationSlug,
            segmentSlug,
            name,
          ).catch((error) => {
            console.log(`Failed to fetch units for a segment state: ${error}`);
            return [];
          }),
        ),
      );

      const verificationUnits: VerificationUnits<TContext, TEvent> = new Map();
      [...verificationColumns.entries()].forEach(([i, name]) => {
        verificationUnits.set(
          name,
          fetchedUnits[i] as SegmentUnit<TContext, TEvent>[],
        );
      });

      setMethodology(data);
      setUnits(verificationUnits);
    };
    f();
  }, [workspaceSlug, methodologySlug, investigationSlug, segmentSlug]);

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

      // Gather all required information and ensure it's all available. If
      // there is something missing simply ignore it.
      if (!methodology) return;

      const {draggableId} = event;
      const sourceId = event?.source?.droppableId;
      const destinationId = event?.destination?.droppableId;
      const index = event?.destination?.index;

      if (!sourceId || !destinationId) return;

      const sourceUnits = units.get(sourceId);
      const destinationUnits = units.get(destinationId);

      if (!sourceUnits || !destinationUnits) return;

      const unit = sourceUnits.find(({id}) => id.toString() === draggableId);

      if (!unit) return;

      // Transition this piece of data to the next state.
      const machine = Machine(methodology.process);
      const currentState = machine.resolveState(unit.state);
      const ev = `to_${destinationId}`.toUpperCase();
      const nextState = machine.transition(currentState, ev);

      if (!nextState.changed) return;

      // Reorder lists of units and update unit with new state.
      unit.state = nextState;
      units.set(
        sourceId,
        sourceUnits.filter(({id}) => id !== unit.id),
      );
      units.set(
        destinationId,
        // Insert unit at the specified index.
        [
          ...destinationUnits.slice(0, index),
          unit,
          ...destinationUnits.slice(index),
        ],
      );

      setUnits(new Map(units.entries()));

      // Persist the new state as a fire and forget action.
      // FIXME: reverse changes when server update fails.
      updateUnitState(
        workspaceSlug,
        investigationSlug,
        segmentSlug,
        unit.id,
        nextState,
      );
    },
    [units, workspaceSlug, investigationSlug, segmentSlug, methodology],
  );

  if (!methodology) return <div />;

  return (
    <div>
      <DragDropContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
        <div className="overflow-x-scroll">
          <div className="flex w-100 h-100">
            {[...units.keys()].map((name, i) => {
              const isDroppable =
                allowedColumns.length === 0 || allowedColumns.includes(name);
              const data = units.get(name) || [];

              return (
                <Droppable key={name} droppableId={name}>
                  {(provided, snapshot) => (
                    <div
                      ref={provided.innerRef}
                      {...provided.droppableProps}
                      className="h-100"
                    >
                      <VerificationColumn
                        className={c("vh-80", i === 0 ? "mr3" : "mh3")}
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
                        <>
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
                        </>
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

export default Verification;
