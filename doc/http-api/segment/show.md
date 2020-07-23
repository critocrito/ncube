# Show a Segment

Show details of a segment.

**URL** : `/workspaces/<workspace>/segment/<segment>`

**Method** : `GET`

## Success Response

The response envelope contains a single segment entity.

**Code** : `200 OK`

**Content examples**

```json
{
  "query": "Aleppo AND Rebels",
  "title": "MF001A",
  "slug": "mf001a",
  "created_at": "2020-06-22T09:20:51Z",
  "updated_at": "2020-07-10T23:00:09.224Z"
}
```
