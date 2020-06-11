# Update the account details

Change details for this account. This request must be authorized.

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

**Code** : `200 OK`

```json
{
  "status": "success",
  "data": "<encrypted password hash>"
}
```

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
