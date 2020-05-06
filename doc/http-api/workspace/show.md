# Show a workspace

Show details of a single workspace.

**URL** : `/workspaces/<workspace_id`

**Method** : `GET`

## Success Response

The response envelope contains a single workspace entity.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": {
    "id": 1,
    "name": "Love is a Battlefield",
    "slug": "love-is-a-battlefield",
    "description": null,
    "created_at": "2020-05-05T11:50:02.868008Z",
    "updated_at": "2020-05-05T11:50:02.868008Z",
    "kind": "local",
    "location": "~/Ncube/love-is-a-battlefield"
  }
}
```
