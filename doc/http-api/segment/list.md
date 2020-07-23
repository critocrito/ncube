# List all segments

List all segments for a workspace

**URL** : `/workspaces/<workspace>/segments`

**Method** : `GET`

## Success Response

The response contains a list of segments for a workspace.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "query": "Aleppo AND Rebels",
      "title": "MF001A",
      "slug": "mf001a",
      "created_at": "2020-06-22T09:20:51Z",
      "updated_at": "2020-07-10T23:00:09.224Z"
    }
  ]
}
```


