# Editor Configuration

## VS Code

VS Code is supported out of the box. The project includes a `.vscode/settings.json` file that configures `rust-analyzer` to match the CI pipeline (running `clippy` with strict checks).

## Neovim

Neovim does not read `.vscode` folders by default. However, you can install a plugin called `neoconf.nvim` that parses `.vscode/settings.json` and applies them to your LSP automatically.

This is the best solution for "configuring it for everyone" because you only maintain the `.vscode/settings.json` file, and both VS Code and Neovim users get the strict checks.

### How to set it up:

1. Install `folke/neoconf.nvim` using your package manager (Lazy, Packer, etc.).
2. Call `require("neoconf").setup()` **BEFORE** you set up `lspconfig` or `rustaceanvim`.

That's it. `neoconf` will see `"rust-analyzer.check.command": "clippy"` in your JSON file and inject it into your Neovim LSP session.
