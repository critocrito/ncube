# List all segments for an investigation

List all segments for an investigation.

**URL** : `/workspaces/<workspace>/investigations/<investigation>/segments`

**Method** : `GET`

## Success Response

The response contains a list of segments.

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
