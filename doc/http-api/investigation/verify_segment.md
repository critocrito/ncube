# Send a segment to an investigation

Send a segment to an investigation to verify data of the segment.

**URL** : `/workspaces/<workspace>/investigations/<investigation>`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "segment": "mf001a"
}
```

## Success Response

**Code** : `201 Created`

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

The segment is already part of the investigation.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Segment already part of investigation."
}
```
