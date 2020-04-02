# Ncube Resources and Entities

## Ncube

The local installation of Ncube on an computer.

**Fields:**

- `db_path` :: Location of the Ncube database.
- `workspace_root` :: Path to the directory holding all local workspaces.
- `name` :: The name of the local user.
- `email` :: The email of the local user.

**Operations:**

- `bootstrap` :: The initial setup of Ncube. This creates a local Ncube database
  to store all configuration.
- `show` :: Display Ncube configuration.
- `edit` :: Update the configuration of an Ncube installation.

**Endpoints:**

- `GET /` :: Display the Ncube configuration.
- `PUT /` :: Update the configuration.
- `POST /` :: Bootstrap the Ncube configuration.

## Workspace

A workspace maps to a data project. It contains a single database and directory,
or maps to a single remote workspace. In the past we called a workspace a
collection.

**Fields:**

- `workspace_type` :: Whether this is a local or remote workspace.

**Fields Local Workspace:**

- `path` :: Location of workspace directory.
- `db_type` :: Use Sqlite (default) or PostgreSQL.
- `db_connection` :: Connection string to database, e.g. `sqlite://path/to/db`.

**Fields of Remote Workspace:**

- `url` :: Full URL of remote backend, e.g. `https://ncube.syrianarchive.org:6666`
- `api_key` :: Authorization token.

**Operations:**

- `create` :: set up a new workspace. A workspace can either be:
  - local :: A local workspace consists of a workspace directory and a workspace
    database.
  - remote :: A remote workspace connects to an API over a network.
- `show` :: Show the details of a workspace.
- `update` :: Change the title or configuration for this workspace.
- `delete` :: remove a local work space or a remote workspace configuration.
- `list` :: Show all available workspaces.

**Endpoints:**

- `POST /workspaces` :: Create a new workspace.
- `GET /workspaces` :: List all workspaces.
- `GET /workspaces/<workspace>` :: Show a single workspace.
- `PUT /workspaces/<workspace>` :: Update a workspace.
- `DELETE /workspaces/<workspace>` :: Delete a workspace.

## Workspace Database

Every local workspace has it's own database.

**Operations:**

- `create` :: Run the schema migrations.
- `upgrade` :: Run schema migrations.

## Workspace Directory

Every local workspace has it's own directory location.

- `create` :: Create a directory, place a `package.json`, install any `npm`
  packages and create the full project layout.

## Query

We call the particular source of data a query. This can be a tweet id, youtube
channel or a website URL. The sources of data are fed into a data process that
then yields data that is originating from this source. Managin queries is as
important to a data project as the data it produces.

**Fields:**

- `type` :: Type of the query.
- `term` :: The source to query.
- `created_at` :: The time the query got created.
- `updated_at` :: The last time the query was used.
- `annotations` :: Annotations attached to the query.

**Operations:**

- `create` :: Add a new query to a workspace database.
- `annotate` :: Queries are immutable but can be overlayed with annotations.
- `delete` :: Remove a query from a workspace database.
- `show` :: Display a single query.
- `list` :: Show all queries.
- `search` :: Show all queries that match a certain query.

**Endpoints:**

- `GET /workspaces/<workspace>/queries` :: List all queries.
- `POST /workspaces/<workspace>/queries` :: Create a new query.
- `GET /workspaces/<workspace>/queries/<query>` :: Show a single query.
- `PUT /workspaces/<workspace>/queries/<query>` :: Update a query.
- `DELETE /workspaces/<workspace>/queries/<query>` :: Remove a query.
- `POST /workspaces/<workspace>/queries/search` :: Search for queries.

## Unit of data

This is the actual data that gets fetched. This can be many different things,
like a tweet, document or youtube video. Ncube relies on Sugarcube to fetch this
data.

**Fields:**

- `id_hash`
- `unit_id`
- `title`
- `description`
- `body`
- `author`
- `author_href`
- `channel`
- `channel_href`
- `created_at`
- `fetched_at`
- `href`
- `downloads`
- `scrapes`
- `queries`
- `annotations`

**Operations:**

- `create` :: Units of data are created through a Sugarcube process. For future
  iterations manual creation would be interesting.
- `annotate` :: Units of data are immutable but can be overlayed with
  annotations.
- `show` :: Display a single unit of data. This includes any media attached to
  the unit.
- `search` :: Show all units of data that match a certain query.
  
**Endpoints:**

- `GET /workspaces/<workspace>/data` :: List all units.
- `GET /workspaces/<workspace>/data/<unit>` :: Show a single unit.
- `POST /workspaces/<workspace>/data/search` :: Search for data.

## Annotation

Data that is fetched is immutable but can be enhanced through annotations. They
are simply one or more additional facts that can be added by the user of Ncube.
Annotations are possible for queries as well. Query annotations are added to a
piece of data at it's time of creation.

FIXME: Maybe it would be interesting to attach annotations to data segments as
well.

**Fields:**

- `type` :: The data type of this annotation (e.g. string, boolean, date, tag)
- `term` :: The content of the annotation.
- `description` :: A descriptive text about the annotation.

**Operations:**

- `create` :: Append an annotation to a query or unit of data.
- `update` :: Change the contents of an annotation.
- `delete` :: Remove an annotation.
- `show` :: Display an annotation.
- `list` :: Show all annotations for a query or unit of data.

**Endpoints:**

- `GET /workspaces/<workspace>/<entity>/annotations` :: List all annotations for
  entity (e.g. data, query).
- `POST /workspaces/<workspace>/<entity>/annotations` :: Create an annotation for
  entity (e.g. data, query).
- `GET /workspaces/<workspace>/<entity>/annotations/<annotation>` :: Show an
  annotation for entity (e.g. data, query).
- `PUT /workspaces/<workspace>/<entity>/annotations/<annotation>` :: Update an
  annotations for entity (e.g. data, query).
- `DELETE /workspaces/<workspace>/<entity>/annotations/<annotation>` :: Delete
  an annotations for entity (e.g. data, query).

## Annotation Template

A annotation template is a list of individual annotation template entries. It
forces a structural approach to enhancing the value of the data and is an
important aspect to a structural verification workflow. Each entry has the
following fields:

**Fields:**

- `type` :: The data type of this annotation (e.g. string, boolean, date, tag)
- `description` :: A descriptive text about the annotation.
- `default` :: A default value for the annotation.
- `is_required` :: Whether the annotation is optional.

**Operations:**

- `create` :: Create a new template for an annotation.
- `show` :: Display a single template for an annotation.
- `update` :: Change the template for an annotation.
- `list` :: List all annotations for a verification stage.

**Endpoints:**

## Investigation

Based on a single data collection many investigations can be conducted. They are
questions that are asked to the data set, a focused narrative that the data set
tells. The outcome of an investigation is a set of data that is verified in the
context of the investigation and that the investigators can make a statement
about.

**Fields:**

- `title` :: The name of the investigation.
- `workflow` :: The workflow attached to the investigation.
- `created_at` :: The of creation of investigation.

**Operations:**

- `create` :: Create a new investigation for a workspace.
- `delete` :: Remove an investigation for a workspace.
- `update` :: Update an investigation for a workspace.
- `show` :: Show an investigation for a workspace.
- `list` :: List all investigations for a workspace.
- `append` :: Append a unit of data to an investigation.

**Endpoints:**

- `POST /workspaces/<workspace>/investigations` :: Create a new investigation.
- `GET /workspaces/<workspaces>/investigations` :: List all investigations.
- `GET /workspaces/<workspace>/investigations/<investigation>` :: Show an
  investigation.
- `PUT /workspaces/<workspace>/investigations/<investigation>` :: Update an
  investigation.
- `POST /workspaces/<workspace>/investigations/<investigation>` :: Add data into
  a verification workflow.
  
_FIXME: I'm still uncertain how to handle the specifics of an investigation
workflow._

- `GET /workspaces/<workspace>/investigations/<investigation>/workflow` :: Show
  the state of the investigation verification workflow.
- `GET /workspaces/<workspace>/investigations/<investigation>/workflow/<stage>`
  :: Show the state of a particular stage of a workflow.
- `PUT /workspaces/<workspace>/investigations/<investigation>/workflow/<unit>`
  :: Update a unit of data in an verification workflow.
  
## Verification Workflow

A verification workflow describes the process that data has to undergo in order
to be regarded as verified. In this process, data has to move between different
stages that represents a human process. During each stage of verification data
can be annotated based on an associated annotation template.

A verification workflow is modeled using a finite state machine and is described
as a set of states and the transitions between them.

**Fields:**

- `template` :: The state machine describing the workflow.

**Operations:**

- `create` :: Add a new verification workflow.

**Endpoints:**

- `POST /workflows` :: Create a new verification workflow.
- `GET /workflows/<workflow>` :: Show a workflow.

## Verification Stage

A state of a verification workflow.

- `list` :: List all units of data that are part of a verification stage.
- `transition` :: Transition a unit of data from one stage to another.

**Endpoints:**

## Data Segment

Data segments allow to formulate groupings of data. Those groupings are the
result applying a search/filter query over the whole data set. There are a few
queries that are treated privileged, e.g. a tag system to group units of data as
associated. Data segments are like queries and units of data another ressource
for investigators. It allows to explore a data set in a repeatable and
consistent fashion over a continous period of time.

**Fields:**

- `type` :: A data segment can either be a simple data filter or a special one
  such as incidents.
- `query` :: The query that yields the contents of the data segment.

**Operations:**

- `create` :: Add a new data segment.
- `delete` :: Remove a data segment.
- `update` :: Update a data segment.
- `show` :: Show a data segment.
- `list` :: List all data segments.

**Endpoints:**

- `POST /workspaces/<workspace>/data-segments` :: Create a new data segment.
- `GET /workspaces/<workspace>/data-segments` :: List all data segments.
- `GET /workspaces/<workspace>/data-segments/<segment>` :: Display a data
  segment.
- `PUT /workspaces/<workspace>/data-segments/<segment>` :: Update a data
  segment.

## Data Process

Data process are responsible to fetch data. It relies on
[Sugarcube](https://sugarcubetools.net/) to do so.

**Fields:**

- `type` :: The name of the data process.
- `requirements` :: The list of capabilities a system has to provide in order to
  run this process.

**Operations:**

- `queue` :: Queue a process up for execution.
- `check` :: Verify dependencies for a data process.
- `configure` :: Set a capability for a data process.

**Endpoints:**

- `GET /workspaces/<workspace>/processes` :: List all data processes.
- `GET /workspaces/<workspace>/processes/<process>` :: Show a single data process.
- `POST /workspaces/<workspace>/processes/<process>` :: Queue up a data process.
- `PUT /workspaces/<workspace>/processes/<process>/capabilities/<capability>` ::
  Set a capability for this data process.

## Capability

A capability is a certain thing that Ncube is able to do. There are roughly two
types of capabilties:

- _host capabilities_ are abilities that are provided by the host system. An
  example would be that the host provides `youtube-dl` in order to download
  videos.
- _ncube capabilities_ are abilities that Ncube can provide if configured. An
  example fot that would be a set of API keys for twitter.

**Fields:**

- `type` :: The name for this capability.
- `term` :: The actual capability, e.g. path to a program or a set of API keys.

**Operations:**

- `list` :: Show all system capabilities.
- `show` :: Display a system capability.
- `update` :: Change the configuration for a capability.
- `verify` :: Verify the validity of a capability.

**Endpoints:**

- `GET /capabilities` :: List all capabilities.
- `GET /capabilities/<capability>` :: Show a single capability.

## Run Queue

Data processes can be long running and inherently asynchronous. To manage system
ressources better data processes are queued up and executed sequentially.

_FIXME: Missing_
