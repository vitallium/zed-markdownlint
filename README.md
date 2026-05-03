# zed-markdownlint

This Zed extension uses the
[`markdownlint-lsp`](https://github.com/vitallium/markdownlint-lsp)
language server so Markdown files get lint diagnostics and quick fixes inside
the editor. The extension installs the server automatically the first time it
runs.

## Override rules

Zed forwards settings from global or
`<workspace>/.zed/settings.json` (per project) to the language server. Add a
section for `markdownlint` and list the rules you want to change under
`settings`.

```json
{
  "lsp": {
    "markdownlint": {
      "settings": {
        "MD013": false,
        "MD041": { "level": "warning" }
      }
    }
  }
}
```

## Ignore files

Use the `ignores` property (a [`markdownlint-cli2` config option](https://github.com/DavidAnson/markdownlint-cli2#markdownlint-cli2jsonc)) to exclude files from linting. Glob patterns are supported.

```json
{
  "lsp": {
    "markdownlint": {
      "settings": {
        "ignores": ["CHANGELOG.md", "docs/generated/**"]
      }
    }
  }
}
```

Alternatively, add `ignores` directly to your project's `.markdownlint-cli2.jsonc` config file.
