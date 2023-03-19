# TypeGenByRequestApi

- set config.json

```json
{
  // type codes into below directory
  "dist_root": "requests/dist",
  "sources": [
    {
      // root struct name
      "name": "JsonPlaceHolder",
      "url": "https://jsonplaceholder.typicode.com/posts/1"
    },
    {
      "name": "GitHubRateLimit",
      "url": "https://api.github.com/rate_limit",
      // set env key
      "basicAuth": {
        "username": "GITHUB_OAUTH_CLIENT_ID",
        "password": "GITHUB_OAUTH_CLIENT_SECRET"
      }
    }
  ]
}
```

- run below command

```
tgreq
```
