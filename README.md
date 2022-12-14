## 🎄 Advent of Code 2022 🎄

These are Rust-language solutions for the [coding-challenge advent
calendar](http://adventofcode.com/2022).  You'll need stable Rust 1.56 and Cargo
to run.

I've tried to make the solutions small and somewhat optimized for speed (so far,
no solution takes more than about a second on an up-to-date machine).  Inputs
are included in text file form and parsed.

### External code used

A custom helper library is used, called `advtools`.  It provides utilities for
easily parsing the input files, which I don't want to rewrite each year, and
access to often used external crates like itertools and rayon.

For tasks that require nontrivial datastructures or algorithms, I try to find
and use a third-party crate to show off the ease of using Rust's crates
infrastructures, e.g. `petgraph`.

### Building/benchmarking

All code is contained in a single Cargo project, with a different binary target
for each day.  Solutions are printed to stdout.

A simple Makefile is also provided in order to run all days.  Just run `make`.

Benchmarks are now provided by criterion.rs, use
```
cargo bench --bench days [filter]
```
to run them.
