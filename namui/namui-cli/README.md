# Namui CLI

## Troubleshooting

-   **If you encounter errors related to `std` or `core` not being found when targeting `wasm32-wasi-web` for `start` or `build` commands:**

    Please run the following command to add the required target:

    ```bash
    rustup target add wasm32-wasip1-threads
    ```
