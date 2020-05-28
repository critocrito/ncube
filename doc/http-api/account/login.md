# Login

Login an account for a remote workspace to retrieve an authorization token.

**URL** : `/workspaces/<workspace_slug>/account`

**Method** : `POST`

**Headers**

- `Content-Type`: must be set to `application/json`

**Data**

```json
{
  "email": "alice@example.org",
  "password": "some secret"
}
```

## Success Response

Upon successful login the response contains an authorization token. The token can be used as the bearer token to authorize follow up requests. Tokens stay valid for up to an hour and are valid for a single remote workspace. Only one valid token exists at a time. If a new succesful login occurs the previous token is invalidated.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6ImVtYWlsQGV4YW1wbGUuY29tIiwiaWF0IjoxNTM4MDgwNDE4LCJleHAiOjE1MzgwODQwMTh9.xMxUBEUsj3_VKfmwEH5Rgdzn7XN3wY5AtwU_1ckcr6w"
  }
}
```
