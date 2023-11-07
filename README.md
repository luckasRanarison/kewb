# Kewb

[![Build/test](https://github.com/luckasRanarison/kewb/actions/workflows/rust.yml/badge.svg)](https://github.com/luckasRanarison/kewb/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/kewb)](https://crates.io/crates/kewb)

Kewb is a library for manipulating and solving the 3x3 Rubiks's cube using Kociemba's [two-phase algorithm](http://kociemba.org/cube.htm). There is also a [CLI](#cli) version which showcases most of kewb features.

Please note that this is still a work in progress and the implementation is not yet efficient. The solver does not currently use symmetric reductions, pre-moves, or multi-threaded search.

## Usage

### Library

See https://docs.rs/kewb/latest/kewb/ for an exhaustive list of APIs provided by kewb.

The solver needs some precomputed data which is represented by the `DataTable` struct. However, generating it takes some amount of time so it's recommended to write it on the disk or bundle it with the executable. You can use the `write_table()` function or the `table` command from `kewb-cli` to generate the table.

```rust
use kewb::{
    error::Error,
    generators::generate_state_cross_solved,
    scramble::{scramble_from_state, scramble_from_str},
    CubieCube, DataTable, FaceCube, Solver,
};

fn main() -> Result<(), Error> {
    // Method 1: Bundling the table in the executable
    // const TABLE_BYTES = include_bytes!("./path_to_file")
    // let table = decode_table(&TABLE_BYTES)?;

    // Method 2: Reading the table from a file
    // let table = read_table("./path_to_file")?;

    // Method 3: Generating the table at runtime (slow)
    let table = DataTable::default();

    let mut solver = Solver::new(&table, 23, None);
    let scramble = scramble_from_str("R U R' U'")?; // vec![R, U, R3, U3]
    let state = CubieCube::from(&scramble);
    let solution = solver.solve(state).unwrap();

    println!("{}", solution);

    solver.clear();

    let faces = "DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL";
    let face_cube = FaceCube::try_from(faces)?;
    let state = CubieCube::try_from(&face_cube)?;
    let solution = solver.solve(state).unwrap();

    println!("{}", solution);

    solver.clear();

    let cross_solved = generate_state_cross_solved();
    let scramble = scramble_from_state(edges_solved, &mut solver)?;

    println!("{:?}", scramble);

    Ok(())
}
```

### CLI

By default, there is no timeout, which means the solver will return the first solution it finds. However, by adding a timeout, the solver will continue searching until the timeout has elapsed and return the shortest solution found or nothing. Specifying a lower search depth can result in better solution quality (around 21 to 23 moves), but it can also make the search slower if the depth is less than 20 moves. Nevertheless, it has been proven that all cases can be solved in [20 moves or fewer](https://www.cube20.org/).

```bash
kewb-cli help
# default values: max = 23, timeout = none, details = false
kewb-cli solve --scramble "R U R' U'" --max 22 --timeout 1 --details
kewb-cli solve -s "R U R' U'" -m 22 -t 1 -d
kewb-cli solve --facelet DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL
# default values: state = random, number = 1, preview = false
kewb-cli scramble -p
kewb-cli scramble -n 5
kewb-cli scramble f2l-solved
# generates the table used by the solver
kewb-cli table ./path_to_file
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
