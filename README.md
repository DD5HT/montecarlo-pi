# A multicore implementation of a Monte Carlo algorithm to calculate π

Calculates Pi with a distributed [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm) and scales adaptivly to the available physical cpu cores.

There are also multiple Implementations in other languages to compare the speed with Rust.

## Example

`time cargo run --release -- -s 50000000000`

    Using 4 physical cores:
    π ~ 3.14158117856

    real    2m14.549s
    user    8m38.850s
    sys     0m0.524s

## Single Benchmarks

Single Benchmarks for 100.000.000 iterations.

### Python 3.6.6 Single Core

    [hendrik@odin bin]$ time ./pi.py
    π ~ 3.1416886

    real    0m48.063s
    user    0m47.999s
    sys     0m0.003s

### ruby 2.5.1p57 Single Core

    [hendrik@odin bin]$ time ./pi.rb
    π ~ 3.141757408582426

    real    0m17.217s
    user    0m17.153s
    sys     0m0.024s

### rustc 1.30.0-nightly (33b923fd4 2018-08-18) Multi-core

#### rustc with release flag + lto

    [hendrik@odin bin]$ time cargo run --release
    π ~ 3.14144664

    real    0m0.425s
    user    0m0.829s
    sys     0m0.041s

#### rustc without release flag

    [hendrik@odin bin]$ time cargo run
    π ~ 3.1415312

    real    0m14.044s
    user    0m53.651s
    sys     0m0.057s

### rustc 1.30.0-nightly (33b923fd4 2018-08-18) Single-core

#### rustc with release flag + lto

    [hendrik@odin bin]$ time cargo run --release
    π ~ 3.14177348

    real    0m0.858s
    user    0m0.837s
    sys     0m0.009s

### gcc -o cpi pi.c -lm -Ofast

    [hendrik@odin bin]$ time ./cpi
    Pi ~ 3.14163

    real    0m1.593s
    user    0m1.576s
    sys     0m0.003s

## Observation

    Using a parallel approach creates zero overhead in Rust! (compare user)

    unoptimized Rust < Python < Ruby < C < optimized single core Rust < optimized mulitcore Rust
    Speedup: 1.000 < 1.118 < 3.116 < 30.043 < 64.123 < 64.742

### Disclaimer

    All Benchmarks were run manually so there is a good chance depending on the load of the computer 
    that your results may vary.
