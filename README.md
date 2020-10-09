# sodoku_wasm

sodoku_wasm is a sudoku solver written in rust. It is compiled to WebAssembly
(wasm) and runs in the browser.


## Dependencies

* Rust and Cargo
  - https://www.rust-lang.org/tools/install
* wasm-pack
  - `cargo install wasm-pack` 


## Building

See the [build.ps1](./build.ps1) script.


## Project Structure

- `./dist`: Latest release build, to be served via gh-pages.
- `./solver`: A "normal" Rust lib crate implementing a sudoku solver.
- `./solver-wasm`: A thin wrapper around the solver crate, compiled to wasm.
- `./web`: Static web assets implementing a simple sudoku UI that calls
  solver-wasm to solve the sudoku.

