# A bloom filter implementation in Rust

## Creation
The user specifies the number of elements to insert in the bloom filter, as well as the desired error probability 
A bloom filter is created based on the theoretical formulas (https://en.wikipedia.org/wiki/Bloom_filter).
The size of the bloom filter is then modified to the closest power of 2, replacing, while inserting, expensive modulo operations with bitwise & on bitmasks.
The element type for this implementation is Rust's u64.

## Insertions
Instead of calling consecutively the hash functions with pipelined input, Kirsch-Mitzenmacher-Optimization is used (https://www.eecs.harvard.edu/~michaelm/postscripts/tr-02-05.pdf).
For insertions, a mixing function and XXHASH (https://github.com/Cyan4973/xxHash) were used.
XXHash is included in this repository. It's compiled using build.rs script, and is called directly from rust code ( but indicated as unsafe to rust compiler).

## Test and Benchmarks
Rust automated tests are offered in lib.rs file.
Benchmarks, following criterion, are offered under the benches directory.

## Compile and Run
```
cargo run --release
Tests: cargo test
Benchmark: cargo bench
```
