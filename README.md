# WinForge 

```` powershell

$env:RUST_LOG="debug"

.\winforge-dev.latest.reg ;; Stop-Process -Name explorer -Force

npm run tauri -- dev

$env:RUST_LOG="debug" ;; npm run tauri -- dev

````


# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

