# A multicore implementation of the Monte Carlo algorithm to calculate π

Calculates Pi with the [Monte Carlo algorithm](https://en.wikipedia.org/wiki/Monte_Carlo_algorithm) and scales adaptivly to the available physical cpu cores.

## Example

`time cargo run --release -- -s 50000000000`
```
Using 4 physical cores:
π ~ 3.14158117856

real    2m14.549s
user    8m38.850s
sys     0m0.524s
```
