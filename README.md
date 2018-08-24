# A multicore implementation of a Monte Carlo algorithm to calculate π

Calculates Pi with a distributed [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm) and scales adaptivly to the available physical cpu cores.

## Example

`time cargo run --release -- -s 50000000000`
```
Using 4 physical cores:
π ~ 3.14158117856

real    2m14.549s
user    8m38.850s
sys     0m0.524s
```

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

### rustc 1.30.0-nightly (33b923fd4 2018-08-18) Multi Core

#### rustc with release flag

    [hendrik@odin bin]$ time cargo run --release
    π ~ 3.14143124

    real    0m0.332s
    user    0m1.099s
    sys     0m0.013s

#### rustc without release flag

    [hendrik@odin bin]$ time cargo run
    π ~ 3.1415312

    real    0m14.044s
    user    0m53.651s
    sys     0m0.057s

### Observation

    unoptimized Rust < Python < Ruby < optimized Rust
    Speedup: 1.000 < 1.118 < 3.116 < 48.818

### Disclaimer

    All Benchmarks were run manually so there is a good chance depending on the load of the computer 
    that your results may vary.
