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

## Hyperfine benchmarks

Hyperfine benchmarks for 100.000.000 iterations.


### Single core Rust

```
Benchmark #1: ./pi
  Time (mean ± σ):      1.326 s ±  0.011 s    [User: 1.321 s, System: 0.001 s]
  Range (min … max):    1.315 s …  1.346 s    10 runs
```

### Single core GCC

```
Benchmark #1: ./target/pi-gcc
  Time (mean ± σ):      1.872 s ±  0.070 s    [User: 1.864 s, System: 0.001 s]
  Range (min … max):    1.811 s …  2.018 s    10 runs
```

### Single core clang

```
Benchmark #1: ./target/pi-clang
  Time (mean ± σ):      1.794 s ±  0.013 s    [User: 1.787 s, System: 0.001 s]
  Range (min … max):    1.780 s …  1.825 s    10 runs
```



