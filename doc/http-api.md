# Ncubed HTTP API

- [Response Envelope](#response-envelope)
  - [Success](#success)
  - [Errors](#errors)
- [Endpoints](#endpoints)
  - [Ncube](#ncube)
  - [Workspaces](#workspaces)
- [Entities](#entities)
  - [Config Setting](#config-setting)
  - [Workspace](#workspace)

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

## Endpoints

### Ncube

- [Show Ncube Configuration](http-api/ncube/show.md) `GET /`
- [Bootstrap Ncube](http-api/ncube/bootstrap.md) `POST /`
- [Update Ncube Configuration](http-api/ncube/update.md) `PUT /`

### Workspaces

- [List all workspaces](http-api/workspace/list.md) `GET /workspaces`
- [Show a workspace](http-api/workspace/show.md) `GET /workspaces/<workspace_id>`
- [Create a workspace](http-api/workspace/create.md) `POST /workspaces`

## Entities

### Config Setting

```json
{
  "name": "workspace_root",
  "description": "The directory where all local workspaces are stored.",
  "required": true,
  "value": "~/ncubed"
}
```

### Workspace

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
