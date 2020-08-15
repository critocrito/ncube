# List units for an investigation segment

List units for that are part of an investigation segment. 

**URL** : `/workspaces/<workspace>/investigations/<investigation>/segments`

**Method** : `GET`

## Query Parameters

**state** : List units limited to units that are in this state.

## Success Response

The response contains a list of contrived units. The unit contains the id, the current state, title, dates and count of different media.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "id": 1663020,
      "source": "youtube_channel",
      "title": "22 6 Homs  أوغاريت حمص حي القصور , دمار هائل في السيارات ومدينة اشباح ج8",
      "videos": 2,
      "images": 0,
      "state": {
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
        "done":false
      }
    }
  ]
}
```
