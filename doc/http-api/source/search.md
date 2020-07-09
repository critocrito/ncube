# Search Sources

Search all sources of a workspace using `query`. The search results can be paginated using the `page` and `size` query parameters.

**URL** : `/workspaces/<workspace_id>/sources/search`

**Method** : `GET`

## Query Parameters

**query** : A URL encoded search query string. This parameter is required.
**page** : The index of the page to retrieve. Defaults to 0.
**size** : The number of units for one page. Defaults to 20.

## Success Response

The response contains a the total number of search results and a single page of sources.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    "total": 1,
    "data": [
      {
        "id": 1,
        "type": "youtube_video",
        "term": "https://youtube.com/watch?v=54aef32",
        "tags": [
          {"label": "incident_code", "description": null}
        ]
      }
    ]
  ]
}
```
