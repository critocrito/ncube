# Ncubed HTTP API

## Response Envelope

A basic HTTP response looks like this:

```json
{
    "status": "success",
    "data": {
        "ncube": {"workspace_root": "/home/crito/ncube"}
    }
}
```

The response envelope always contains a status key and some additional key
containing data of the response.

|`status`|Description|Required Keys|Optional Keys|
|-------|-----------|-------------|-------------|
|`success`|All went well.|`status`, `data`| |
|`error`|An error happened processing the request.|`status`,`errors`| |

### Success

The `success` envelope contains the status `success` and a data field that can
either be an object or an array of objects.

```json
{
    "status": "success".
    "data": {
        // ... Data fields ...
    }
}
```

### Errors

The `error` envelope contains the status `error` and an errors object containing
all errors.

```json
{
    "status": "error",
    "errors": [
        // ... Error messages
    ]
}
```

## Endpoints

* [Show Ncube Configuration](http-api/ncube/get.md) `GET /`
* [Bootstrap Ncube](http-api/ncube/post.md) `POST /`
