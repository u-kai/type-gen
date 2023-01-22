# sf-df

## What is

- This crate do source lang(or json) file to dist lang(or json) file

## How to use

- You write sf-df.json like below

```json
{
  "src": {
    "root": "./jsons",
    "extension": "json"
  },
  "dist": {
    "root": "./src/structs",
    "extension": "rs"
  }
}
```

- And run this command

```shell
./sf-df
```

- Create to dist files

- If you customize source and dist setting, add sf-df.json like bleow

```json
{
  "dist": {
    "root": "./src/structs",
    "extension": "rs"
  },
  "rust": {
    "all_optional": true,
    "all_visibility": "pub",
    "add_derives_with_serde": ["Clone", "Debug"]
  }
}
```
