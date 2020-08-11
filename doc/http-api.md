# Ncubed HTTP API

- [Response Envelope](#response-envelope)
  - [Success](#success)
  - [Errors](#errors)
- [Authorization](#authorization)
- [Endpoints](#endpoints)
  - [Ncube](#ncube)
  - [Account](#account)
  - [Workspaces](#workspaces)
  - [Sources](#sources)
  - [Units](#units)
  - [Segments](#segments)
  - [Processes](#processes)
  - [Methodologies](#methodologies)
  - [Investigations](#investigations)
  - [Stats](#stats)
- [Entities](#entities)
  - [Config Setting](#config-setting-entity)
  - [Account](#account-entity)
  - [Workspace](#workspace-entity)
  - [Source](#source-entity)
  - [Unit](#unit-entity)
  - [Segment](#segment-entity)
  - [Process](#process-entity)
  - [Investigation](#investigation-entity)
  - [Methodology](#methodology-entity)
  - [Stat](#stat-entity)

## Response Envelope

A basic HTTP response looks like this:

```json
{
  "status": "success",
  "data": {
    "ncube": {"workspace_root": "/home/crito/ncube"}
  }
}
```

The response envelope always contains a status key and some additional key
containing data of the response.

| `status`  | Description                               | Required Keys     | Optional Keys |
| --------- | ----------------------------------------- | ----------------- | ------------- |
| `success` | All went well.                            | `status`, `data`  |               |
| `error`   | An error happened processing the request. | `status`,`errors` |               |

### Success

The `success` envelope contains the status `success` and a data field that can
either be an object or an array of objects.

```json
{
    "status": "success".
    "data": {
        // ... Data fields ...
    }
}
```

### Errors

The `error` envelope contains the status `error` and an errors object containing
all errors.

```json
{
  "status": "error",
  "errors": [
    // ... Error messages
  ]
}
```

## Authorization

To access remote workspaces a valid account has to be provided by the operator
of the remote Ncube installation. Accounts are authenticated using an email
address and a password. The local Ncube installation will automatically login
when accessing the remote workspace. The remote Ncube installation will return
an [JSON Web Tokens](https://jwt.io/) token when the login succeeds.

[JWT](https://jwt.io/) as described in
[RFC7519](https://tools.ietf.org/html/rfc7519) is used to authorize all
requests. A new token can be retrieved by [logging in](#account). When
successful, a valid token is returned that be used to authorize requests to any
protected route. A token stays valid for one hour. A new token has to be
requested by logging in once the previous token expires.

The token has to be set using the `Authorization` HTTP header. The authorization
type is `Bearer` and the credential is the JWT token.

```
GET /protected/route
Content-Type: application/json
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6ImVtYWlsQGV4YW1wbGUuY29tIiwiaWF0IjoxNTM4MDgwNDE4LCJleHAiOjE1MzgwODQwMTh9.xMxUBEUsj3_VKfmwEH5Rgdzn7XN3wY5AtwU_1ckcr6w
```

If no token was supplied or the token didn't verify, the API returns a **401
Unauthorized** response.

Only access to remote workspaces require authorization; requests to the local
Ncube installation (originating from `127.0.0.1`) don't.

## Endpoints

### Ncube

- [Show Ncube Configuration](http-api/ncube/show.md) `GET /`
- [Bootstrap Ncube](http-api/ncube/bootstrap.md) `POST /`
- [Update Ncube Configuration](http-api/ncube/update.md) `PUT /`

### Account

- [Login](http-api/account/login.md) `POST /workspaces/<workspace_slug>/account`
- [Update the account details](http-api/account/update.md) `PUT /workspaces/<workspace_slug>/account`

### Workspaces

- [List all workspaces](http-api/workspace/list.md) `GET /workspaces`
- [Show a workspace](http-api/workspace/show.md) `GET /workspaces/<workspace_id>`
- [Create a workspace](http-api/workspace/create.md) `POST /workspaces`

### Sources

- [List all sources](http-api/source/list.md) `GET /workspaces/<workspace_id>/sources`
- [Create a query](http-api/source/create.md) `POST /workspaces/<workspace_id>/sources`
- [Show a query](http-api/source/show.md) `POST /workspaces/<workspace_id>/sources/<source_id>`
- [Update a sources](http-api/source/update.md) `PUT /workspaces/<workspace_id>/sources/<source_id>`
- [Remove a sources](http-api/source/remove.md) `DELETE /workspaces/<workspace_id>/sources/<source_id>`
- [Search for sources](http-api/source/search.md) `GET /workspaces/<workspace_id>/sources/search`
- [List all source tags](http-api/source/tags.md) `GET /workspaces/<workspace_id>/source-tags`

### Units

- [List all units](http-api/data/list.md) `GET /workspaces/<workspace_id>/data`
- [Search for units](http-api/data/search.md) `GET /workspaces/<workspace_id>/data/search`

### Segments

- [Create a segment](http-api/segment/create.md) `POST /workspaces/<workspace>/segments`
- [List all segments](http-api/segment/list.md) `GET /workspaces/<workspace>/segments`
- [Show a segment](http-api/segment/show.md) `GET /workspaces/<workspace>/segments/<segment>`
- [Remove a segment](http-api/segment/remove.md) `DELETE /workspaces/<workspace>/segments/<segment>`
- [Update a segment](http-api/segment/update.md) `PUT /workspaces/<workspace>/segments/<segment>`

### Processes

- [List all processes](http-api/process/list.md) `GET /workspaces/<workspace>/processes`
- [Configure a process](http-api/process/configure.md) `PUT /workspaces/<workspace>/processes`
- [Run a process](http-api/process/run.md) `POST /workspaces/<workspace>/processes`

### Methodologies

- [List all methodologies](http-api/methodology/list.md) `GET /workspaces/<workspace>/methodologies`
- [Show a methodology](http-api/methodology/show.md) `GET /workspaces/<workspace>/methodologies/<methodology>`
- [Create a methodology](http-api/methodology/create.md) `POST /workspaces/<workspace>/methodologies`

### Investigations

- [List all investigations](http-api/investigation/list.md) `GET /workspaces/<workspace>/investigations`
- [Show an investigation](http-api/investigation/show.md) `GET /workspaces/<workspace>/investigations/<investigation>`
- [Create an investigation](http-api/investigation/create.md) `POST /workspaces/<workspace>/investigations`

### Stats

- [Total count sources](http-api/stat/total_sources.md) `GET /workspaces/<workspace_id>/stats/sources/total`
- [Types of sources](http-api/stat/types_sources.md) `GET /workspaces/<workspace_id>/stats/sources/types`
- [Total count units](http-api/stat/total_units.md) `GET /workspaces/<workspace_id>/stats/data/total`
- [Count of unit sources](http-api/stat/sources_units.md) `GET /workspaces/<workspace_id>/stats/data/sources`
- [Count of videos](http-api/stat/videos_units.md) `GET /workspaces/<workspace_id>/stats/data/videos`
- [Total count of units of a segment](http-api/stat/total_units_segment.md) `GET /workspaces/<workspace_id>/stats/segments/<segment_id>/units`

## Entities

### Config Setting Entity

```json
{
  "name": "workspace_root",
  "description": "The directory where all local workspaces are stored.",
  "required": true,
  "value": "~/ncubed"
}
```

### Account Entity

```json
{
  "email": "alice@example.org",
  "name": "Alice"
}
```

### Workspace Entity

```json
{
  "id": 1,
  "name": "Syrian Archive",
  "slug": "syrian-archive",
  "description": null,
  "created_at": "2020-05-05T11:50:02.868008Z",
  "updated_at": "2020-05-05T11:50:02.868008Z",
  "kind": "local",
  "location": "~/Ncube/syrian-archive"
}
```

### Source Entity

```json
{
  "id": 1,
  "type": "youtube_video",
  "term": "https://youtube.com/watch?v=54aef32",
  "tags": [
    {"label": "Alice", "description": null},
    {"label": "MF00AAs", "description": "Some option description."}
  ]
}
```

### Unit Entity

```json
{
  "id": 1663020,
  "id_hash": "0000535cbbb696136d6841c4449a2125e2d222e9630c938e89279c7d9598c7ef",
  "content_hash": "7c23c855d747baee9e6bc75426733bf44ae2c340749a08d62433d027da8f6b80",
  "source": "youtube_channel",
  "unit_id": "H3TE-LC_BGQ",
  "body": null,
  "href": "https://www.youtube.com/watch?v=H3TE-LC_BGQ",
  "author": "شبكة أوغاريت الإخبارية - سوريا | Ugarit News - Syria",
  "title": "22 6 Homs  أوغاريت حمص حي القصور , دمار هائل في السيارات ومدينة اشباح ج8",
  "description": null,
  "language": null,
  "created_at": "2012-06-22T09:20:51Z",
  "fetched_at": "2017-11-10T23:00:09.224Z",
  "media": [
    {
      "id_hash": "ad425f074edb4543f007312240aa1a4d8a2a9f93ac1fab9d2cac9cea8f207837",
      "type": "image",
      "term": "https://i.ytimg.com/vi/H3TE-LC_BGQ/hqdefault.jpg"
    },
    {
      "id_hash": "9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504",
      "type": "video",
      "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ"
    },
    {
      "id_hash": "423d225fb29c9a34fd7a0364cfc2f4fd2882d4fe373d20ac735cdd24eb6e6b4c",
      "type": "image",
      "term": "https://i.ytimg.com/vi/H3TE-LC_BGQ/hqdefault.jpg"
    },
    {
      "id_hash": "b02f6add30abd83a2ea4ada59ceb70188f736be02315bee9b4f02d1233f66086",
      "type": "url",
      "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ"
    }
  ],
  "downloads": [
    {
      "id_hash": "9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504",
      "type": "video",
      "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ",
      "md5": "b449d98880a778d44d8e57cbaf330c7d",
      "sha256": "aafde02ff8ece69667e5f5211c72bfab6f85065983582f91d963e1c4872b9c6a",
      "location": "data/0000535cbbb696136d6841c4449a2125e2d222e9630c938e89279c7d9598c7ef/youtubedl/9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504.mp4"
    }
  ],
  "sources": [
    {
      "id": 2288,
      "type": "youtube_channel",
      "term": "UCvQaT359f3FFSlp_dBwBC4A",
      "tags": []
    }
  ]
}
```

### Segment Entity

```json
{
  "query": "Aleppo AND Rebels",
  "title": "MF001A",
  "slug": "mf001a",
  "created_at": "2020-06-22T09:20:51Z",
  "updated_at": "2020-07-10T23:00:09.224Z"
}
```

### Process Entity

```json
{
  "name": "Youtube Video",
  "description": "Fetch individual Youtube videos.",
  "config": [
    {
      "name": "Youtube API Key",
      "kind": "secret",
      "value": {"api_key": "some key"}
    },
    {
      "name": "Other API Key",
      "kind": "secret",
      "value": null
    }
  ]
}
```

### Methodology

```json
{
  "id": 1,
  "name": "Tutorial",
  "slug": "tutorial",
  "description": "Attacks against hospital attacks.",
  "process": {}
}
```

### Investigation Entity

```json
{
  "id": 1,
  "name": "Hospital Attacks",
  "slug": "hospital-attacks",
  "description": "Attacks against hospital attacks.",
  "methodology": "tutorial-methodology"
}
```

### Stat Entity

```json
{
  "name": "count_sources",
  "value": 9000
}
```
