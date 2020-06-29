import {parseISO} from "date-fns";
import React from "react";

import Sidebar from "../../common/sidebar";
import {Workspace} from "../../types";

const workspaces: Workspace[] = [
  {
    id: 1,
    name: "Syrian Archive",
    description: "",
    slug: "syrian-archive",
    kind: "local",
    location: "~/Ncube/syrian-archive",
    database: "sqlite",
    database_path: "~/Ncube/syrian-archive/sugarcube.db",
    created_at: parseISO("2020-06-19T11:00:09.856432Z"),
    updated_at: parseISO("2020-06-19T11:00:09.856432Z"),
  },
  {
    id: 2,
    name: "Yemen Archive",
    description: "",
    slug: "yemen-archive",
    kind: "local",
    location: "~/Ncube/yemen-archive",
    database: "sqlite",
    database_path: "~/Ncube/yemen-archive/sugarcube.db",
    created_at: parseISO("2020-06-19T11:00:09.856432Z"),
    updated_at: parseISO("2020-06-19T11:00:09.856432Z"),
  },
  {
    id: 2,
    name: "A workspace with a really long name",
    description: "",
    slug: "long-name-archive",
    kind: "local",
    location: "~/Ncube/yemen-archive",
    database: "sqlite",
    database_path: "~/Ncube/yemen-archive/sugarcube.db",
    created_at: parseISO("2020-06-19T11:00:09.856432Z"),
    updated_at: parseISO("2020-06-19T11:00:09.856432Z"),
  },
];

export default (
  <div className="noto lh-copy flex flex-column">
    <Sidebar workspaces={workspaces} />
  </div>
);
