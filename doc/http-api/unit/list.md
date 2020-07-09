# List all units

List all units for a workspace. The list of units can be paginated using the `page` and `size` query parameters.

**URL** : `/workspaces/<workspace_id>/data`

**Method** : `GET`

## Query Parameters

**page** : The index of the page to retrieve. Defaults to 0.
**size** : The number of units for one page. Defaults to 20.

## Success Response

The response contains a the total number of units for a workspace and a single page of units.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    "total": 1,
    "data": [
      {
        "id": 1663020,
        "id_hash": "0000535cbbb696136d6841c4449a2125e2d222e9630c938e89279c7d9598c7ef",
        "content_hash": "7c23c855d747baee9e6bc75426733bf44ae2c340749a08d62433d027da8f6b80",
        "source": "youtube_channel",
        "unit_id": null,
        "body": null,
        "href": "https://www.youtube.com/watch?v=H3TE-LC_BGQ",
        "author": "شبكة أوغاريت الإخبارية - سوريا | Ugarit News - Syria",
        "title": "22 6 Homs  أوغاريت حمص حي القصور , دمار هائل في السيارات ومدينة اشباح ج8",
        "description": null,
        "language": null,
        "created_at": "2012-06-22T09:20:51Z",
        "fetched_at": "2017-11-10T23:00:09.224Z",
        "media": [
          {
            "id_hash": "ad425f074edb4543f007312240aa1a4d8a2a9f93ac1fab9d2cac9cea8f207837",
            "type": "image",
            "term": "https://i.ytimg.com/vi/H3TE-LC_BGQ/hqdefault.jpg"
          },
          {
            "id_hash": "9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504",
            "type": "video",
            "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ"
          },
          {
            "id_hash": "423d225fb29c9a34fd7a0364cfc2f4fd2882d4fe373d20ac735cdd24eb6e6b4c",
            "type": "image",
            "term": "https://i.ytimg.com/vi/H3TE-LC_BGQ/hqdefault.jpg"
          },
          {
            "id_hash": "b02f6add30abd83a2ea4ada59ceb70188f736be02315bee9b4f02d1233f66086",
            "type": "url",
            "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ"
          }
        ],
        "downloads": [
          {
            "id_hash": "9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504",
            "type": "video",
            "term": "https://www.youtube.com/watch?v=H3TE-LC_BGQ",
            "md5": "b449d98880a778d44d8e57cbaf330c7d",
            "sha256": "aafde02ff8ece69667e5f5211c72bfab6f85065983582f91d963e1c4872b9c6a",
            "location": "data/0000535cbbb696136d6841c4449a2125e2d222e9630c938e89279c7d9598c7ef/youtubedl/9216c4c829df41c22033857d379fe23fcac769bcbafe18286fa9dcfe38ffc504.mp4"
          }
        ],
        "sources": [
          {
            "id": 2288,
            "type": "youtube_channel",
            "term": "UCvQaT359f3FFSlp_dBwBC4A",
            "tags": []
          }
        ]
      }
    ]
  ]
}
```
