# Register a Client

When starting Ncube register as a client in order to receive push updates. There are two scenarios, either register the UI as a client of the Ncube host, or register a Ncube host as a client of a remote Ncube host.

**URL** : `/register`

**Method** : `POST`

**Example Request**

```json
POST / register
```

## Success Response

A succesful registration will yield a unique UUID for this client and a path to the websocket endpoint to subscribe for push notifications.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": {
    "uuid": "b4f3f00a0bde4342973f47e911346eb2",
    "url": "ws://127.0.0.1:40666/ws/b4f3f00a0bde4342973f47e911346eb2"
  }
}
```
