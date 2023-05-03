# Kewb

## Description

A Rubik's cube solver using [Kociemba's two phase algorithm](http://kociemba.org/cube.htm). This is still a work in progress and this is not an efficent implementation yet, the solver doesn't use symmetric reductions and pre-move.

## Usage

By default there is no timeout which means the solver will return the first solution it will find, by adding a timeout the solver will continue searching until the timeout has been ellapsed and return the shortest solution found. Specifying lower solution length can result into faster search but can make the search slower if too low, but it has been proved that all cases can be solved in 20 moves or less.

```bash
kewb help
kewb solve --scramble "R U R' U'" --max 22 --timeout 1 --details
# defaults max: 23, timeout: none, details: false
```

## Build

**NB: You must have the rust toolchain installed**

Clone the repository and run:

```bash
cargo build --release
```

## Testing

You can run the tests by running:

```bash
cargo test --workspace --lib
```

## Todo

-   [ ] Fancy CLI scramble view
-   [ ] Algorithm optimization
-   [ ] Server and webview

# References

-   Two phase algorithm overview: http://kociemba.org/cube.htm

-   Two phase algorithm implementation in python: https://qiita.com/7y2n/items/55abb991a45ade2afa28
