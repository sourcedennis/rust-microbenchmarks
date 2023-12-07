# Rust Microbenchmarks

A collection of small benchmarks written in Rust. Mostly for performance, sometimes for other metrics.

## Motivation

Often, my workflow is:

* In some project, I require functionality X
* I benchmark multiple implementations of X in an isolated project Y
* I copy the "best" implementation from Y to the main project
* I throw away Y

Thus, I lose the *reason* for deciding on a particular implementation. In this project, I keep that history around whenever I need to *reconvince* myself (or others) of a decision.

## Conclusions

* `parse-u64s` - Parsing *textual* `u64`s is faster with [`nom`](https://docs.rs/nom/latest/nom/) than with `&str::parse()`.
* `hashset-u32` - A `HashSet<u32>` is much faster when using [`fxhash`](https://docs.rs/fxhash/latest/fxhash/) rather than when using no hash ([`nohash-hasher`](https://docs.rs/nohash-hasher/latest/nohash_hasher/)).

## License

BSD-3 - See the `LICENSE` file
