# 🎄 Andrei's 2021 Advent of Code 🎄
![Build Status](https://github.com/AndreiBarsan/2021-advent-of-code/actions/workflows/aoc-ci-build.yml/badge.svg)
## Learning Goals
 - [X] Rust basics (vectors, arrays, math, etc.)
 - [X] Rust basic CLI
 - [X] Rust linear algebra and ndarrays (e.g., https://github.com/rust-ndarray/ndarray)
 - [X] Simple GitHub Actions set up (just linting initially)
 - [X] Automatic linting and formatting (trunk?) Used `cargo fmt -- --check` to ensure unformatted code cannot pass the build.
 - [ ] Finish all AoC problems (hints are OK from Dec 12 on, in the second half)
 - [ ] Basic unit testing
 - [ ] Rust <> C++ interop
 - [ ] Advanced lifetime concepts using this tutorial: https://rust-unofficial.github.io/too-many-lists/second-final.html

## Bonus Goals
 - [ ] Call PyTorch from Rust
 - [ ] `nom` (tutorial: https://blog.logrocket.com/parsing-in-rust-with-nom/)


## Running the Code

Assuming [Cargo](https://doc.rust-lang.org/rust-by-example/cargo.html) has been set up, to run a problem, simply use
the following:
```
cargo run --release --bin <XX_problem>
```
The above should automatically build the code with its dependencies, and run the appropriate problem.

## Highlights

 * I am particularly proud of my [Day 19 (Beacon Scanner) solution](src/bin/19_beacon_scanner.rs). I tackled the problem
   as a computer vision challenge and ended up implementing very simple feature (triangle) detection and matching
   followed by building a pose graph to transform all beacon readings (point clouds) into the same coordinate frame. I
   leveraged the `nalgebra` crate a fair bit, and the final version solves the task in under 3ms on a 2019 Macbook Pro
   with an i9 9880H (not counting disk IO ---if you do, the run time increases to about 14ms).

## Learnings
 - Powerful type-safe, efficient, support for ndarrays, but still at times much more verbose than numpy.
 For instance, computing the median of an array is unnecessarily complicated.
 - Same for dealing with NaNs. Very rigorous but kind of annoying for simple data science or ML workloads.
 - Keep the naive implementation arround ALWAYS. Do not simply rewrite it - you can use it to debug your fast
   implementation. For instance, your fast implementation may work for the demo input in Part 2 but not for the puzzle
   input.
    - Often you can debug parts of the internal state from your fast implementation using your naive one!
    - Example: Problem 14 - Polymerization, where you used your naive implementation to fix the fast one by looking at
      the character histograms produced by the two to identify a counting bug in the letter counting function of the
      fast implementation.