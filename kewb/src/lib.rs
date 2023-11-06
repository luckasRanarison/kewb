//! A crate for manipulating and solving the 3x3 Rubik's cube with [Kociemba's two phase algorithm](http://kociemba.org/cube.htm).

pub(crate) mod cube;
pub(crate) mod two_phase;

pub use cube::{cubie::CubieCube, facelet::Color, facelet::FaceCube, moves::Move};
pub use two_phase::solver::{Solution, Solver};
pub use two_phase::utils::DataTable;

/// Module containing 3x3 cube constants.
pub mod constants {
    pub use crate::cube::constants::*;
}

/// Module containing table read and write operations.
pub mod fs {
    pub use crate::two_phase::fs::*;
}

/// Module for generating moves table.
pub mod move_table {
    pub use crate::two_phase::moves::*;
}

/// Module for generating pruning table.
pub mod pruning_table {
    pub use crate::two_phase::pruning::*;
}

/// Module for translating permutations and orientations into the two phase algorithm coordinate.
pub mod index {
    pub use crate::cube::index::*;
}

/// Some utility functions.
pub mod utils {
    pub use crate::cube::moves::scramble_from_string;
}

/// Module containing functions for generating some cubie level states.
pub mod generators {
    pub use crate::cube::generators::*;
}

pub mod error;
