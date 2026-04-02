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

## Troubleshooting

-   **If you encounter errors related to `std` or `core` not being found when targeting `wasm32-wasi-web` for `start` or `build` commands:**

    Please run the following command to add the required target:

    ```bash
    rustup target add wasm32-wasip1-threads
    ```
