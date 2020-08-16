/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import React, {useCallback, useEffect, useState} from "react";
import {DragDropContext} from "react-beautiful-dnd";
import {EventObject, Machine} from "xstate";

import {
  listSegmentUnitsByState,
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
import VerificationColumn from "./verification-column";

interface VerificationProps {
  workspace: Workspace;
  investigation: Investigation;
  segment: Segment;
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
  investigation: {
    title: investigationTitle,
    methodology: methodologySlug,
    slug: investigationSlug,
  },
  segment: {title: segmentTitle, slug: segmentSlug},
}: VerificationProps) => {
  const [methodology, setMethodology] = useState<
    Methodology<TContext, TStateSchema, TEvent>
  >();
  const [units, setUnits] = useState<VerificationUnits<TContext, TEvent>>(
    new Map(),
  );
  const [highlightedColumn, setHighlightedColumn] = useState<
    string | undefined
  >();
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

      const verificationColumns = ["incoming_data"]
        .concat(Object.keys(states))
        .concat(["discarded_data", "verified_data"]);

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

  const onDragUpdate = useCallback((event) => {
    const destinationId: string = event?.destination?.droppableId;

    setHighlightedColumn(destinationId);
  }, []);

  const onDragEnd = useCallback(
    (event) => {
      // We remove the column highlighting in any case.
      setHighlightedColumn(undefined);
      setAllowedColumns([]);

      // Gather all required information and ensure it's all available. If
      // there is something missing simply ignore it.
      if (!methodology) return;

      const {draggableId, index} = event;
      const sourceId = event?.source?.droppableId;
      const destinationId = event?.destination?.droppableId;

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
        // Insert unit at the speciifed index.
        destinationUnits
          .slice(0, index)
          .concat([unit])
          .concat(destinationUnits.slice(index)),
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
      <h2>
        Verifying {investigationTitle}/{segmentTitle}
      </h2>

      <DragDropContext
        onDragStart={onDragStart}
        onDragUpdate={onDragUpdate}
        onDragEnd={onDragEnd}
      >
        <div className="overflow-x-scroll">
          <div className="flex w-100 h-100">
            {[...units.keys()].map((name, i) => {
              const isHighlighted = highlightedColumn === name;
              const isDroppable =
                allowedColumns.length === 0 || allowedColumns.includes(name);
              return (
                <VerificationColumn
                  key={name}
                  className={c(i === 0 ? "mr3" : "mh3")}
                  name={name}
                  units={units.get(name) || []}
                  isHighlighted={isHighlighted}
                  isDroppable={isDroppable}
                />
              );
            })}
          </div>
        </div>
      </DragDropContext>
    </div>
  );
};

export default Verification;
