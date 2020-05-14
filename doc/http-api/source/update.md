# Update a source

Modify an existing source.

**URL** : `/workspaces/<workspace_id>/sources/<source_id>`

**Method** : `PUT`

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

**Code** : `204 No Content`

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
