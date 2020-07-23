# Create a Segment

Create a new segment for a workspace.

**URL** : `/workspaces/<workspace_id>/segments`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "query": "Aleppo AND Rebels",
  "title": "MF001A"
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

Another segment with the same slug already exists.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Segment `mf001a` already exists."
}
```
