# Create a new workspace

Create a new workspace for the local Ncube configuration.

**URL** : `/workspaces`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "name": "Syrian Archive",
  "description": "A longer description of your workspace.",
  "kind": "local"
}
```

Valid values for `kind` are `local` and `remote`.

**Example Request**

```json
POST /
Content-Type: application/json

{
    "name": "Syrian Archive",
    "description": "A longer description of your workspace.",
    "kind": "local"
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
