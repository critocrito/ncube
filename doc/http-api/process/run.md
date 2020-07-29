# Run a Process

Initiate a process run.

**URL** : `/workspaces/<workspace>/processes`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "key": "youtube_video",
  "kind": "all"
}
```

A data process has three `kind` variants, `all`, `new` and `selection`.

## Success Response

**Code** : `201 Created`

**Headers**

- `Location`: The location of the new workspace.

## Error Response

### `400 Bad request`

The data process requires configuration/

**Content Example**

```json
{
  "status": "error",
  "errors": "Process lacks configuration."
}
```
