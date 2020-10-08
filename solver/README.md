# Solver

Solver implements a simple recursive brute-force sudoku solver.


## Tests

* Unit tests in [src/lib.rs](./src/lib.rs)
* Integration tests in [tests/integration_test.rs](./tests/integration_test.rs)

Run via:
```
cargo test
```

## Benchmark

* Benchmark uses the [Criterion.rs](https://crates.io/crates/criterion) crate
* Located in [benches/solve_benchmark.rs](./benches/solve_benchmark.rs)

Run via:

```
cargo bench
```

## Profiling in Visual Studio 2019

Build benchmark binary:

```
cargo bench --no-run
```

Then select the binary via the Visual Studio Performance Profiler:
1. Open Visual Studio 2019
2. Select "Continue without code"
3. Debug -> Performance Profiler...
4. Choose Target -> Executable
   - Path: `[...]\target\release\deps\solve_benchmark-[hash].exe`,
     pick most recent.
   - Command line: `--bench --profile-time 10`
     (see [Criterion Profiling](https://bheisler.github.io/criterion.rs/book/user_guide/profiling.html))
   - Working directory: `[...]\target\release\deps`
5. Start profiling

Note that Criterion does a warmup phase first aprox. 3 sec, make sure to
exclude that time range when looking at the profile results.
