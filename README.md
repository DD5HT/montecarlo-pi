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

### go build pi.go

    [hendrik@odin bin]$ time ./pi
    Pi: +3.141755e+000

    real    0m2.553s
    user    0m2.545s
    sys     0m0.004s


## perf stat for Rust

    Performance counter stats for './pi':
           875,521733      task-clock:u (msec)       #    1,000 CPUs utilized
                    0      context-switches:u        #    0,000 K/sec
                    0      cpu-migrations:u          #    0,000 K/sec
                  132      page-faults:u             #    0,151 K/sec
        2.701.546.266      cycles:u                  #    3,086 GHz
        9.614.782.386      instructions:u            #    3,56  insn per cycle
        1.081.446.108      branches:u                # 1235,202 M/sec
              399.899      branch-misses:u           #    0,04% of all branches
        0,875843969 seconds time elapsed
        0,874664000 seconds user
        0,000000000 seconds sys

            Performance counter stats for './pi -m':
                   876,914303      task-clock:u (msec)       #    1,928 CPUs utilized
                            0      context-switches:u        #    0,000 K/sec
                            0      cpu-migrations:u          #    0,000 K/sec
                          177      page-faults:u             #    0,202 K/sec
                2. 90.776.793      cycles:u                  #    3,068 GHz
                9.615.095.565      instructions:u            #    3,57  insn per cycle
                1.081.503.908      branches:u                # 1233,306 M/sec
                      404.617      branch-misses:u           #    0,04% of all branches
                0,454751231 seconds time elapsed
                0,876609000 seconds user
                0,000000000 seconds sys
## Observation

    Using a parallel approach creates zero overhead in Rust! (compare user)

    unoptimized Rust < Python < Ruby < C < optimized single core Rust < optimized mulitcore Rust
    Speedup: 1.000x < 1.118x < 3.116x < 21.081x < 30.043x < 64.123x < 64.742x

### Disclaimer

    All Benchmarks were run manually so there is a good chance depending on the load of the computer 
    that your results may vary.
