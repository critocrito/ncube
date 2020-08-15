# Update the process state of a unit

Update the state of a unit of data. The payload is a serialized
[Xstate](https://xstate.js.org/) state. The API does no validation on the
content of the payload. As long as it is a valid JSON it's alright for the API.

**URL** : `/workspaces/<workspace>/investigations/<investigation>/segments/<segment>/<unit>`

**Method** : `PUT`

**Headers**

- `Content-Type`: must be set to `application/json`.

**Data**

```json
{
  "actions": [],
  "activities": {},
  "meta": {},
  "events": [],
  "value": "incoming_data",
  "_event": {
    "name": "xstate.init",
    "data": {
      "type": "xstate.init"
    },
    "$$type": "scxml",
    "type": "external"
  },
  "_sessionid": null,
  "event": {
    "type": "xstate.init"
  },
  "children": {},
  "done": false
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
