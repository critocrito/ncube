# List all methodologies

List all available methodologies for a workspace.

**URL** : `/workspaces/<workspace>/methodologies`

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
      "title": "Tutorial",
      "slug": "tutorial",
      "description": "Attacks against hospital attacks.",
      "process": {}
    }
  ]
}
```
