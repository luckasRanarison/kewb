pub(crate) mod cube;
pub(crate) mod two_phase;

pub use cube::{moves::Move, state::State};
pub use two_phase::solver::{solve, Solution, Solver};

pub mod fs {
    pub use crate::two_phase::fs::*;
}

pub mod move_table {
    pub use crate::two_phase::moves::*;
}

pub mod pruning_table {
    pub use crate::two_phase::pruning::*;
}

pub mod index {
    pub use crate::cube::index::*;
}

pub mod utils {
    pub use crate::cube::{moves::scramble_from_string, utils::generate_random_state};
}
