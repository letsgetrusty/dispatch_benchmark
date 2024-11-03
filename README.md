# Dispatch Benchmarking

This is a simple benchmark to compare the performance of different dispatch methods in Rust. 
Specifically, we are comparing the performance of v-tables (used by trait objects) vs. jump tables (used by enums).

# Running the benchmark
```bash
$ cargo bench
```