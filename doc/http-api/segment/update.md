# Update a Segment

Modify an existing segment. When updating the title as well the resource location will change since the segment slug will be updated as well.

**URL** : `/workspaces/<workspace>/sources/<segment>`

**Method** : `PUT`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "query": "Aleppo AND Rebels",
  "title": "MF001A"
}
```

## Success Response

**Code** : `204 No Content`

## Error Response

### `400 Bad Request`

The requested workspace does not exist.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Workspace `my-workspace` doesn't exist."
}
```

### `400 Bad Request`

Another segment with the same slug already exists.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Segment `mf001a` already exists."
}
```
