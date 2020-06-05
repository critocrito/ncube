# Create a new workspace

Create a new workspace for the local Ncube configuration. This endpoint responds
only to local requests.

**URL** : `/workspaces`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

Valid values for `kind` are `local` and `remote`. Workspaces of kind `remote` have an additional `endpoint` field containing the HTTP URL of the remote workspace.

Valid values for `database` are `sqlite` and `http`. The database path for Sqlite databases are automatically generated. `http` databases require the `kind` field to be set to `remote`. It is an error to use a different database than `http` for remote workspaces. The database path is derived from the `endpoint` field.

```json
{
  "name": "Syrian Archive",
  "description": "A longer description of your workspace.",
  "kind": "local",
  "database": "sqlite"
}
```

```json
{
  "name": "Syrian Archive",
  "description": "A longer description of your workspace.",
  "kind": "remote",
  "endpoint": "https://example.org/workspaces/syrian-archive",
  "database": "http"
}
```

**Example Request**

```json
POST /
Content-Type: application/json

{
  "name": "Syrian Archive",
  "description": "A longer description of your workspace.",
  "kind": "local",
  "database": "sqlite"
}
```

## Success Response

**Code** : `201 Created`

**Headers**

- `Location`: The location of the new workspace.

## Error Response

### `400 Bad request`

Data provided fails the validation.

**Content Example**

```json
{
  "status": "error",
  "errors": {
    "kind": ["Valid values are either `local` or `remote`"]
  }
}
```
