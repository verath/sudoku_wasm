# sodoku_wasm

## Dependencies

* Rust and Cargo
  - https://www.rust-lang.org/tools/install
* wasm-pack
  - `cargo install wasm-pack` 


## Compiling to WebAssembly

```
wasm-pack build --target web --out-dir ../web/pkg solver-wasm
```
