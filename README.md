# RustBrowser

A Rust-first browser project inspired by Chromium architecture.

## Current version

The first usable browser shell now includes:

- native Rust window
- Chromium-based WebView2 page rendering on Windows through Wry
- address bar
- URL navigation
- search from the address bar
- Back
- Forward
- Reload
- automatic address-bar updates during navigation
- responsive webview resizing

Wry uses Microsoft Edge WebView2 as its web engine on Windows, so this stage gives RustBrowser a real Chromium-based rendering engine without pretending that the whole Chromium engine has already been rewritten in Rust.

## Run

```bash
cargo run
```

For a release executable:

```bash
cargo build --release
```

## Architecture

```
RustBrowser
├── Browser shell
│   ├── Window
│   ├── Toolbar
│   ├── Navigation
│   └── Tabs (foundation)
├── WebView2 / Wry
├── Network (next)
├── Renderer integration (next)
├── IPC
├── GPU
└── Sandbox
```

## Roadmap

1. Browser toolbar and navigation
2. Multi-tab system
3. Downloads
4. Bookmarks and history
5. Settings
6. Developer tools
7. Rust networking layer
8. Rust-owned browser services and IPC
9. Progressive replacement of platform/Chromium components with Rust
10. Opera GX-style gaming features and side panel
