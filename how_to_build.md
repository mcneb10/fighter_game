# How to build
Compile using

```shell
wasm-pack build --target web --release
```

You may need to set `OPENSSL_DIR` if on macOS to the cellar location in homebrew.

Then copy `executor_wasm/pkg/executor_wasm.js` and `executor_wasm/pkg/executor_wasm_bg.wasm` to `resources/static`