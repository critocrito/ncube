CREATE TABLE IF NOT EXISTS methodology (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  description TEXT,
  process JSON NOT NULL,
  initial_state JSON NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS investigation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  description TEXT,
  methodology INTEGER NOT NULL REFERENCES methodology(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE INDEX IF NOT EXISTS investigation_methodology_idx ON investigation (methodology);

insert into methodology (
  title,
  slug,
  description,
  process,
  initial_state,
  created_at,
  updated_at
) VALUES (
  'Tutorial',
  'tutorial',
  'The methodology used in the Ncube tutorial at https://sugarcubetools.net/ncube/tutorial.',
  '{"id":"tutorial","initial":"incoming_data","states":{"incoming_data":{"on":{"TO_DESK_RESEARCH":"desk_research","TO_DISCARDED_DATA":"discarded_data"}},"discarded_data":{"on":{"TO_INCOMING_DATA":"incoming_data"}},"verified_data":{"on":{"TO_INCOMING_DATA":"incoming_data","TO_DISCARDED_DATA":"discarded_data"}},"desk_research":{"on":{"TO_SIGN_OFF":"sign_off","TO_DISCARDED_DATA":"discarded_data"},"meta":{"annotations":[{"name":"location","description":"Longer description","kind":"string"},{"name":"narrative","description":null,"kind":"text","required":true}]}},"sign_off":{"on":{"TO_VERIFIED_DATA":"verified_data","TO_DESK_RESEARCH":"desk_research","TO_DISCARDED_DATA":"discarded_data"}}}}',
  '{"actions":[],"activities":{},"meta":{},"events":[],"value":"incoming_data","_event":{"name":"xstate.init","data":{"type":"xstate.init"},"$$type":"scxml","type":"external"},"_sessionid":null,"event":{"type":"xstate.init"},"children":{},"done":false}',
  '2020-08-13T07:54:40.659291+00:00',
  '2020-08-13T07:54:40.659291+00:00'
);
