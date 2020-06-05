# Show Ncube Configuration

Display the configuration of the Ncube host application. This endpoint responds
only to local requests.

**URL** : `/`

**Method** : `GET`

## Success Response

Ncube configuration settings are represented as a list of objects. Each setting
has a name, value, description and flag whether the setting is required. Only
settings that have been set are displayed.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "name": "workspace_root",
      "description": "The directory where all local workspaces are stored.",
      "required": true,
      "value": "~/ncubed"
    },
    {
      "name": "email",
      "description": "The email address of the local Ncube user.",
      "required": false,
      "value": "alice@example.org"
    }
  ]
}
```

## Error Response

### `404 Not Found`

The Ncube configuration is missing. This means that Ncube hasn't been
boostrapped yet.
