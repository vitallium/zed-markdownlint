# zed-markdownlint

This Zed extension uses the
[`markdownlint-lsp`](https://github.com/vitallium/markdownlint-lsp)
language server so Markdown files get lint diagnostics and quick fixes inside
the editor. The extension resolves the server in this order: the configured
`lsp.markdownlint.binary.path`, `markdownlint-lsp-server` on the worktree PATH,
then an automatically installed managed `markdownlint-lsp` package.

To use a specific local server executable, configure its path:

```json
{
  "lsp": {
    "markdownlint": {
      "binary": {
        "path": "/path/to/markdownlint-lsp-server"
      }
    }
  }
}
```

Zed adds `--stdio` by default. If `binary.arguments` is configured, it replaces
that default, including when set to an empty array. Set `binary.env` to pass
environment variables to the executable.

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
