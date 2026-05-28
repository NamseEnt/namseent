# Namui CLI

## Prerequisites

### Linux → Windows cross-compilation

To cross-compile for `x86_64-pc-windows-msvc` from Linux, install the following:

```bash
# System packages
sudo apt install build-essential clang lld llvm

# Symlinks (cargo-xwin expects these names without version suffix)
sudo ln -sf /usr/bin/clang-cl-19 /usr/bin/clang-cl
sudo ln -sf /usr/bin/llvm-lib-19 /usr/bin/llvm-lib

# Rust target and cargo-xwin
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin
```

### macOS → Windows cross-compilation

To cross-compile for `x86_64-pc-windows-msvc` from macOS, install the following:

```bash
# LLVM (provides clang-cl, llvm-lib, lld-link)
brew install llvm

# Make clang-cl / llvm-lib / lld-link discoverable on PATH.
# Add to your shell rc (zshrc/bashrc):
export PATH="$(brew --prefix llvm)/bin:$PATH"

# Rust target and cargo-xwin
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin
```

## Project icon

Declare a source PNG in `Cargo.toml`:

```toml
[package.metadata.namui]
icon = "asset/image/icon/app.png"   # square PNG, recommended 1024x1024 RGBA
```

When building `x86_64-pc-windows-msvc`, namui-cli:

-   Generates a multi-size `.ico` (16/32/48/64/128/256) and embeds it into the
    `.exe` via a `build.rs` using `embed-resource`.
-   Emits Steam Partner upload assets to `target/namui/steam-assets/`:
    -   `shortcut-icon-512.png` (Steamworks "Shortcut Icon")
    -   `app-icon-184.jpg` (Steamworks "App Icon"; transparent pixels composited
        onto black)

These assets are *not* uploaded automatically — Steamworks does not expose a
public asset-upload API. Upload them once via the Steamworks Partner web site.

The source PNG must be square and at least 256×256. A warning is printed if it
is smaller than 1024×1024 (downscaling stays sharp; upscaling does not).

## Troubleshooting

-   **If you encounter errors related to `std` or `core` not being found when targeting `wasm32-wasi-web` for `start` or `build` commands:**

    Please run the following command to add the required target:

    ```bash
    rustup target add wasm32-wasip1-threads
    ```

-   **If `embed-resource` fails to find a Windows resource compiler when cross-compiling:**

    Make sure `llvm-rc` is on PATH (provided by `brew install llvm` on macOS,
    `apt install llvm` on Linux).
