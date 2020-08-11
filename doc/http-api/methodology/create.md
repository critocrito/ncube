# Create a methodology

Create a new methodology for a workspace.

**URL** : `/workspaces/<workspace>/methodologies`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "title": "Tutorial",
  "description": null,
  "process": {}
}
```

## Success Response

**Code** : `201 Created`

## Error Response

### `400 Bad Request`

The requested workspace does not exist.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Workspace `my-workspace` doesn't exist."
}
```

### `400 Bad Request`

Another methodology with the same slug already exists.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Methodology `tutorial` already exists."
}
```
