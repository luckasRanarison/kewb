use crate::{
    cube::moves::Move::{self, *},
    move_table::MoveTable,
    pruning_table::PruningTable,
};

use bincode::{Decode, Encode};

pub const ALL_MOVES: [Move; 18] = [
    U, U2, U3, D, D2, D3, R, R2, R3, L, L2, L3, F, F2, F2, B, B3, B2,
];
pub const PHASE2_MOVES: [Move; 10] = [U, U2, U3, D, D2, D3, R2, L2, F2, B2];

pub type Table<T> = Vec<Vec<T>>;

/// Contains the move and prunning table used by the two-phase algorithm
#[derive(Default, Encode, Decode)]
pub struct DataTable {
    pub move_table: MoveTable,
    pub pruning_table: PruningTable,
}
