# Kewb

[![Build/test](https://github.com/luckasRanarison/kewb/actions/workflows/rust.yml/badge.svg)](https://github.com/luckasRanarison/kewb/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/kewb)](https://crates.io/crates/kewb)

This is a Rubik's cube solver that uses Kociemba's [two-phase algorithm](http://kociemba.org/cube.htm) and can also be used as a library for manipulating the 3x3 Rubik's cube. However, please note that this is still a work in progress and the implementation is not yet efficient. The solver does not currently use symmetric reductions, pre-moves, or multi-threaded search.

## Usage

### CLI

By default, there is no timeout, which means the solver will return the first solution it finds. However, by adding a timeout, the solver will continue searching until the timeout has elapsed and return the shortest solution found or nothing. Specifying a lower search depth can result in better solution quality (around 21 to 23 moves), but it can also make the search slower if the depth is less than 20 moves. Nevertheless, it has been proven that all cases can be solved in [20 moves or fewer](https://www.cube20.org/).

```bash
kewb-cli help
kewb-cli solve --scramble "R U R' U'" --max 22 --timeout 1 --details
kewb-cli solve -s "R U R' U'" -m 22 -t 1 -d
# default values: max = 23, timeout = none, details = false
kewb-cli scramble
kewb-cli scramble -n 5
# default values: number = 1
kewb-cli solve --facelet DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL
```

### Library

https://docs.rs/kewb/latest/kewb/

```rust
use kewb::{error::Error, utils::scramble_from_string, DataTable, FaceCube, Solver, State};

fn main() -> Result<(), Error> {
    // Generating the table takes some time so it's recommended to write it on the disk
    // write_table("path_to_file")?;
    // let table = read_table("./path_to_file")?;
    let table = DataTable::default();

    let mut solver = Solver::new(&table, 23, None);
    let scramble = scramble_from_string("R U R' U'").unwrap(); // vec![R, U, R3, U3]
    let state = State::from(&scramble);
    let solution = solver.solve(state).unwrap();

    println!("{}", solution);

    let faces = "DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL";
    let face_cube = FaceCube::try_from(faces)?;
    let state = State::try_from(&face_cube)?;
    let solution = solver.solve(state).unwrap();

    println!("{}", solution);

    Ok(())
}
```

## Build

**NB: You must have the rust toolchain installed**

Clone the repository and run:

```bash
cargo build --release # build in target/build/
# or
cargo install --path ./kewb-cli # install to ~/.cargo/bin/
```

## Testing

You can run the tests by running:

```bash
cargo test --lib
```

## Todo

- [x] Add Documentation
- [ ] More CLI features
- [ ] Algorithm optimization

## References

- Two phase algorithm overview: http://kociemba.org/cube.htm

- Two phase algorithm implementation in python: https://qiita.com/7y2n/items/55abb991a45ade2afa28
