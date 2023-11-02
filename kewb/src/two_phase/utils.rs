use crate::{
    cube::moves::Move::{self, *},
    move_table::MoveTable,
    pruning_table::PruningTable,
};
use bincode::{Decode, Encode};

/// Corner orientation count
pub const CO_COUNT: u16 = 2187;
/// Edge orientation count
pub const EO_COUNT: u16 = 2048;
/// E-slice edge combination count
pub const E_COMBO_COUNT: u16 = 495;

/// Corner premutation count
pub const CP_COUNT: u16 = 40320;
/// U-D layer edge premutation count
pub const UD_EP_COUNT: u16 = 40320;
/// E-slice edge permutation count
pub const E_EP_COUNT: u16 = 24;

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
