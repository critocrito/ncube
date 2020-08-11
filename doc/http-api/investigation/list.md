# List all investigations

List all available investigations for a workspace.

**URL** : `/workspaces/<workspace>/investigations`

**Method** : `GET`

## Success Response

The response contains a list of investigations available for a workspace.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Hospital Attacks",
      "slug": "hospital-attacks",
      "description": "Attacks against hospital attacks.",
      "methodology": "tutorial-methodology",
      "segments": [
        {
          "id": 1,
          "query": "Aleppo AND Rebels",
          "title": "MF001A",
          "slug": "mf001a",
          "created_at": "2020-06-22T09:20:51Z",
          "updated_at": "2020-07-10T23:00:09.224Z"
        }
      ]
    }
  ]
}
```
