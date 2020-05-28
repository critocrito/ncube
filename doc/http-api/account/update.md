# Update the account details

Change details for this account.

**URL** : `/workspaces/<workspace_slug>/account`

**Method** : `PUT`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "email": "alice@example.org",
  "password": "some secret password"
}
```

**Authorization**: required

## Success Response

**Code** : `204 No Content`

## Error Response

### `400 Bad request`

Data provided fails the validation.

**Content Example**

```json
{
  "status": "error",
  "errors": {
    "email": ["Format not a valid email address."]
  }
}
```
