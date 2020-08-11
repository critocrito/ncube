# Create an investigation

Create a new investigation for a workspace.

**URL** : `/workspaces/<workspace>/investigations`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "title": "Hospital Attacks",
  "description": null,
  "methodology": "tutorial-methodology"
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

Another investigation with the same slug already exists.

**Content Example**

```json
{
  "status": "error",
  "code": 400,
  "errors": "Investigation `hospital-attacks` already exists."
}
```
