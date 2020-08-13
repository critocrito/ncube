# Methodology Data Format

Every methodology is described as a state machine and an annotation schema for every state. There are three predefined states that are part of every methodology: `incoming_data`, `discarded_data` and `verified_data`. Other states can be added as needed per methodology. Only user defined states can have an annotation schema.

There is a convention to name transitions betweent state with capital letters and starting with `TO_`, followed by the name of the state in capital letters. So, to transition to the state `desk_research`, name the transition `TO_DESK_RESEARCH`. There are three predefined transitions:

- `TO_DISCARDED_DATA`
- `TO_INCOMING_DATA`
- `TO_VERIFIED_DATA`

The following is a complete example of a user defined methodology. It contains the three required states and two additional custom states, `desk_research` and `sign_off`. The `desk_research` state defines and optional annotation schema.

```json
{
  "id": "tutorial",

  "initial": "incoming_data",

  "states": {
    "incoming_data": {
      "on": {
        "TO_DESK_RESEARCH": "desk_research",
        "TO_DISCARDED_DATA": "discarded_data"
      }
    },

    "discarded_data": {
      "on": {
        "TO_INCOMING_DATA": "incoming_data"
      }
    },

    "verified_data": {
      "on": {
        "TO_INCOMING_DATA": "incoming_data",
        "TO_DISCARDED_DATA": "discarded_data"
      }
    },

    "desk_research": {
      "on": {
        "TO_SIGN_OFF": "sign_off",
        "TO_DISCARDED_DATA": "discarded_data"
      },
      "meta": {
        "annotations": [
          {
            "name": "location",
            "description": "Longer description",
            "kind": "string"
          },
          {
            "name": "narrative",
            "description": null,
            "kind": "text",
            "required": true
          }
        ]
      }
    },

    "sign_off": {
      "on": {
        "TO_VERIFIED_DATA": "verified_data",
        "TO_DESK_RESEARCH": "desk_research",
        "TO_DISCARDED_DATA": "discarded_data"
      }
    }
  }
}
```

The above methodology format can be parsed by [`xstate`](https://xstate.js.org/), the library Ncube uses to execute the state machines. Below is an example session for reference how such an execution looks like.

```js
const machine = xstate.Machine(cfg);

const {initialState} = machine;
// Current state
console.log(initialState.value);
let next = machine.transition(initialState, "TO_VERIFIED_DATA");
// Did the transition work?
console.log(next.changed);

// Transition for real now
next = machine.transition(next, "TO_DESK_RESEARCH");
console.log(next.value, next.changed);

// annotations attached on each state
console.log(next.meta[`${machine.id}.${next.value}`]);

// possible next transitions
console.log(next.nextEvents);

const [ev] = next.nextEvents;

next = machine.transition(next, ev);
console.log(next.value);

// serialize current state
const persisted = JSON.stringify(next);
console.log(persisted);

// deserialize current state
const state = machine.resolveState(JSON.parse(persisted));
console.log(state.value);
```
