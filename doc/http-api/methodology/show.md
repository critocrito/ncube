# Show a methodology

Show the details of a methodology.

**URL** : `/workspaces/<workspace>/methodologies/<methodology>`

**Method** : `GET`

## Success Response

The response envelope contains a single methodology entity.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": {
    "id": 1,
    "title": "Tutorial",
    "slug": "tutorial",
    "description": "Attacks against hospital attacks.",
    "process": {}
  }
}
```
