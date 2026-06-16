# chroma-ls

Simple LSP server implementation for parsing color literals
via the textDocument/documentColor method.

## Supported color formats

	#RRGGBB
	#RRGGBBAA

## Editor Configuration

### Neovim

```lua
vim.lsp.config('chroma_ls', {
  cmd = { 'chroma-ls' }
  -- With no filetypes provided, server auto-attaches to every buffer
})

vim.lsp.enable('chroma_ls')
```

### Helix

In `languages.toml`:

```toml
[language-server.chroma-ls]
command = "chroma-ls"
```

Helix [does not currently](https://github.com/helix-editor/helix/issues/12721)
support assigning an LSP globally to all filetypes.
You need to specify the languages explicitly. For example:

```toml
[[language]]
name = "json"
language-servers = [ "chroma-ls" ]
```
