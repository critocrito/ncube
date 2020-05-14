# List all sources

List all sources for a workspace.

**URL** : `/workspaces/<workspace_id>/sources`

**Method** : `GET`

## Success Response

The response contains a list of sources of a workspace.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "type": "youtube_video",
      "term": "https://youtube.com/watch?v=54aef32",
      "annotations": [
        { "id": 1, "type": "tag", "term": "#incident_code" },
        { "id": 2, "type": "tag", "term": "#other_code" }
      ]
    }
  ]
}
```
