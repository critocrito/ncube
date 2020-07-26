# List all processes

List all processes and process configurations for a workspace

**URL** : `/workspaces/<workspace>/processes`

**Method** : `GET`

## Success Response

The response contains a list of processes for a workspace.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Youtube Video",
      "description": "Fetch individual videos from Youtube.",
      "config": [
        {
          "name": "Youtube API Key",
          "key": "youtube",
          "description": "Youtube API credentials.",
          "kind": "secret",
          "template": {
            "api_key": "Youtube API key"
          },
          "value": null
        }
      ]
    }
  ]
}
```
