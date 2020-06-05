# Bootstrap Ncube

Create the initial configuration (bootstrap) of the Ncube host application. This
endpoint responds only to local requests.

**URL** : `/`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
[
  { "name": "workspace_root", "value": "~/ncubed" },
  { "name": "email", "value": "alice@example.org" }
]
```

**Example Request**

```json
POST /
Content-Type: application/json

[
    {"name": "workspace_root", "value": "~/ncubed"},
    {"name": "email", "value": "alice@example.org"}
]
```

## Success Response

**Code** : `201 Created`

**Headers**

- `Location`: The location where to query the current Ncube configuration.

## Error Response

### `400 Bad request`

Data provided fails the validation.

**Content Example**

```json
{
  "status": "error",
  "errors": {
    "workspace_root": ["This field is required."]
  }
}
```

### `405 Method Not Allowed`

Ncube can be bootstrapped only once. Attempting to bootstrap Ncube multiple
times lead to this error.
