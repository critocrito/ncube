# List all workspaces

List all available workspaces.

**URL** : `/workspaces`

**Method** : `GET`

## Success Response

The response contains a list of workspaces configured for Ncube.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Love is a Battlefield",
      "slug": "love-is-a-battlefield",
      "description": null,
      "created_at": "2020-05-05T11:50:02.868008Z",
      "updated_at": "2020-05-05T11:50:02.868008Z",
      "kind": "local",
      "location": "~/Ncube/love-is-a-battlefield"
    },
    {
      "id": 2,
      "name": "Total Eclipse of the heart",
      "slug": "total-eclipse-of-the-heart",
      "description": null,
      "created_at": "2020-05-05T12:38:23.836290Z",
      "updated_at": "2020-05-05T12:38:23.836290Z",
      "kind": "local",
      "location": "~/Ncube/total-eclipse-of-the-heart"
    }
  ]
}
```
