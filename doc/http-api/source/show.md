# Show a Source

Show details of a single source.

**URL** : `/workspaces/<workspace_id>/sources/<source_id>`

**Method** : `GET`

## Success Response

The response envelope contains a single source entity.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": {
    "id": 1,
    "type": "youtube_video",
    "term": "https://youtube.com/watch?v=54aef32",
    "tags": [{"label": "incident_code", "description": null}]
  }
}
```
