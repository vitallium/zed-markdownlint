# zed-markdownlint

This Zed extension uses the
[`markdownlint-lsp`](https://github.com/markdownlint/markdownlint-cli2/tree/main/packages/markdownlint-lsp)
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
