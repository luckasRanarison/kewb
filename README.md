# Kewb

## Description

A Rubik's cube solver using Kociemba's two phase algorithm. This is still a work in progress, this is not an efficent implementation yet, the solver doesn't use symetric reductions and pre-move.

## Usage

```bash
kewb help
kewb solve --scramble "R U R' U'" --max 22 --timeout 1 --details
# defaults max: 23, timeout: 1s, details: false
```

## Build

**NB: You must have the rust toolchain installed**

Clone the repository and run

```bash
cargo build
```

## Todo

-   [ ] Fancy CLI scramble view
-   [ ] Algorithm optimization
-   [ ] Server and webview
