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
- [Entities](#entities)
  - [Config Setting](#config-setting-entity)
  - [Account](#account-entity)
  - [Workspace](#workspace-entity)
  - [Query](#query-entity)
  - [Annotation](#annotation-entity)

## Response Envelope

A basic HTTP response looks like this:

```json
{
  "status": "success",
  "data": {
    "ncube": { "workspace_root": "/home/crito/ncube" }
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
Unauthorized** response. If the token is valid but the user lacks access rights
the API returns a **403 Forbidden**.

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
- [Update a sources](http-api/source/update.md) `PUT /workspaces/<workspace_id>/sources/<query_id>`
- [Remove a sources](http-api/source/remove.md) `DELETE /workspaces/<workspace_id>/sources/<query_id>`

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
  "annotations": [
    { "id": 1, "type": "tag", "term": "#incident_code" },
    { "id": 2, "type": "tag", "term": "#other_code" }
  ]
}
```

### Annotation Entity

```json
{
  "id": 1,
  "type": "tag",
  "term": "#incident_code"
}
```
