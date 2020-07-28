# Configure a Process

Update the configuration for a process. Updating the configuration of one
process updates the same capability for other processes of the same workspace as
well.

**URL** : `/workspaces/<workspace>/processes`

**Method** : `PUT`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "name": "Youtube API key",
  "value": {"api_key": "some key"}
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
