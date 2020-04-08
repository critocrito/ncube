# Bootstrap Ncube

Create the initial configuration of the Ncube host application.

**URL** : `/`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
    "workspace_root": "String",
    "name": "String, Optional",
    "email": "String, Optional"
}
```

**Example Request**

```json
POST /
Content-Type: application/json

{
    "workspace_root": "~/ncube",
    "name": "Christo",
    "email": "christo@cryptodrunks.net"
}
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
