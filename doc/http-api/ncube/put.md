# Updates Ncube Settings

Update one or more settings.

**URL** : `/`

**Method** : `PUT`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

One or more settings can be updated at once. Settings are always wrapped in an
array, even if there is only a single setting to update.

```json
[
    {"name": "workspace_root", "value": "~/Desktop/Ncube"},
    {"name": "email", "value": "bob@example.org"}
]
```

**Example Request**

```json
PUT /
Content-Type: application/json

[
    {"name": "email", "value": "alice@example.org"}
]
```

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

### `405 Method Not Allowed`

This error is triggered in case a setting is updated without Ncube being
bootstrapped.
