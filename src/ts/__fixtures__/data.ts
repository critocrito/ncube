import {
  Source,
  AnnotationSchema,
  Process,
  Workspace,
  SourceTag,
  Methodology,
  MethodologySchema,
  EventObject,
  Segment,
} from "../types";

export type FixtureMethodology = Methodology<
  unknown,
  MethodologySchema,
  EventObject
>;

export const localWorkspace: Workspace = {
  id: 23,
  name: "My Workspace",
  slug: "my-workspace",
  created_at: new Date("2020-07-05T09:50:43.089Z"),
  updated_at: new Date("2020-07-05T09:50:43.089Z"),
  database: "sqlite",
  database_path: "sugarcube.db",
  kind: "local" as const,
  location: "project-dir",
  is_created: true,
};

export const remoteWorkspace: Workspace = {
  id: 2,
  name: "the-a-team",
  slug: "the-a-team",
  description: undefined,
  created_at: new Date("2020-07-21T19:52:36.117786Z"),
  updated_at: new Date("2020-07-21T19:52:36.117786Z"),
  kind: "remote" as const,
  location: "https://ncube.cryptodrunks.net",
  database: "http",
  database_path: "https://ncube.cryptodrunks.net",
  is_created: true,
};

export const process1: Process = {
  id: 1,
  key: "youtube_video",
  name: "Youtube Video",
  description: "Preserve individual videos from Youtube.",
  config: [
    {
      name: "Youtube API Key",
      key: "youtube",
      kind: "secret",
      description: "Youtube API credentials.",
      template: {api_key: "Youtube API key"},
      value: undefined,
    },
  ],
};

export const process2: Process = {
  id: 2,
  key: "youtube_video",
  name: "Youtube Video",
  description: "Preserve individual videos from Youtube.",
  config: [
    {
      name: "Youtube API Key",
      key: "youtube",
      kind: "secret",
      description: "Youtube API credentials.",
      template: {api_key: "Youtube API key"},
      value: {api_key: "some key"},
    },
  ],
};

export const sourceTags: SourceTag[] = [
  {label: "MF00023a", description: "Reference code."},
  {label: "MF00022a", description: "Reference code."},
  {label: "MF00021a", description: "Reference code."},
  {label: "MF000235", description: "Reference code."},
  {label: "MF00028a", description: "Reference code."},
  {label: "MF00043a", description: "Reference code."},
  {label: "MF00045a", description: "Reference code."},
  {label: "MF000111", description: "Reference code."},
  {label: "MF000300a", description: "Reference code."},
  {label: "MF0002a", description: "Reference code."},
  {label: "Alice"},
  {label: "Jane", description: "Record Owner."},
  {label: "Check Later"},
];

export const methodology1: FixtureMethodology = {
  id: 1,
  title: "Tutorial",
  slug: "tutorial",
  description: "The methodology used in the Ncube tutorial.",
  process: {
    id: "tutorial",
    initial: "incoming_data",
    states: {
      incoming_data: {
        on: {
          TO_DESK_RESEARCH: "desk_research",
          TO_DISCARDED_DATA: "discarded_data",
        },
      },
      discarded_data: {on: {TO_INCOMING_DATA: "incoming_data"}},
      verified_data: {
        on: {
          TO_INCOMING_DATA: "incoming_data",
          TO_DISCARDED_DATA: "discarded_data",
        },
      },
      desk_research: {
        on: {TO_SIGN_OFF: "sign_off", TO_DISCARDED_DATA: "discarded_data"},
        meta: {
          annotations: [
            {
              name: "location",
              description: "Longer description",
              kind: "string",
            },
            {
              name: "narrative",
              description: null,
              kind: "text",
              required: true,
            },
          ],
        },
      },
      sign_off: {
        on: {
          TO_VERIFIED_DATA: "verified_data",
          TO_DESK_RESEARCH: "desk_research",
          TO_DISCARDED_DATA: "discarded_data",
        },
      },
    },
  },
  created_at: "2020-08-13T07:54:40.659291Z",
  updated_at: "2020-08-13T07:54:40.659291Z",
};

export const segments: Segment[] = [
  {
    id: 1,
    slug: "mf001a",
    title: "MF001A",
    query: "Aleppo AND rebels",
    created_at: "2020-07-24T07:10:10.885295Z",
    updated_at: "2020-07-24T07:10:10.885295Z",
  },
  {
    id: 2,
    slug: "rebel-attacks",
    title: "Rebel Attacks",
    query: "Aleppo OR rebels NOT russia",
    created_at: "2020-07-24T07:10:10.885295Z",
    updated_at: "2020-07-24T07:10:10.885295Z",
  },
];

export const annotations: AnnotationSchema[] = [
  {
    key: "location",
    name: "Location",
    description: "Where did the event take place?",
    kind: "string",
    required: true,
  },
  {
    key: "date-time",
    name: "Date and Time",
    description: "Which day and time did this event take place?",
    kind: "datetime",
  },
  {
    key: "has-mounted-police",
    name: "Mounted Police",
    description: "Can you observe mounted police?",
    kind: "boolean",
  },
  {
    key: "has-police-dog",
    name: "Police Dogs",
    description: "Can you observe police with dogs?",
    kind: "boolean",
  },
  {
    key: "target",
    name: "Attacked Target",
    description: "Who was the target of the attack?",
    kind: "selection",
    selections: ["civilian", "journalist"],
  },
];

export const source: Source = {
  id: 22164,
  type: "youtube_video",
  term: "https://www.youtube.com/watch?v=cHAEAKF2jzo",
  tags: [
    {
      label: "Arnold",
      description: "Schwarzenegger",
    },
    {
      label: "Aleppo",
      description: "location",
    },
  ],
};
