# Create a source

Add a new source to a workspace.

**URL** : `/workspaces/<workspace_id>/sources`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "type": "youtube_video",
  "term": "https://youtube.com/watch?v=123456",
  "annotations": []
}
```

## Success Response

**Code** : `201 Created`

**Headers**

- `Location`: The location of the new source.

## Error Response

### `400 Bad request`

Data provided fails the validation.

**Content Example**

```json
{
  "status": "error",
  "errors": ["Source `kind` and `term` must be unique."]
}
```
