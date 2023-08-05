use super::{moves::*, utils::*};
use crate::cube::moves::Move;
use bincode::{Decode, Encode};

/// Collection of pruning table for filtering unsolvable state at a given depth.
#[derive(Encode, Decode)]
pub struct PruningTable {
    pub co_e: Table<u8>,
    pub eo_e: Table<u8>,
    pub cp_e: Table<u8>,
    pub ep_e: Table<u8>,
}

impl PruningTable {
    pub fn new() -> Self {
        Self {
            co_e: get_prune_table(get_co_table, get_e_combo_table, &ALL_MOVES),
            eo_e: get_prune_table(get_eo_table, get_e_combo_table, &ALL_MOVES),
            cp_e: get_prune_table(get_cp_table, get_e_ep_table, &PHASE2_MOVES),
            ep_e: get_prune_table(get_ud_ep_table, get_e_ep_table, &PHASE2_MOVES),
        }
    }
}

pub type TableGetter = fn() -> Table<u16>;

pub fn get_prune_table(getter_1: TableGetter, getter_2: TableGetter, moves: &[Move]) -> Table<u8> {
    let table_1 = getter_1();
    let table_2 = getter_2();
    let len_1 = table_1.len();
    let len_2 = table_2.len();
    let fill_size = len_1 * len_2;
    let mut pruning_table = vec![vec![255; len_2]; len_1];
    let mut distance = 0;
    let mut filled: usize = 1;

    pruning_table[0][0] = 0;

    while filled != fill_size {
        for i in 0..len_1 {
            for j in 0..len_2 {
                if pruning_table[i][j] == distance {
                    for (m, _) in moves.iter().enumerate() {
                        let next_1 = table_1[i][m] as usize;
                        let next_2 = table_2[j][m] as usize;
                        if pruning_table[next_1][next_2] == 255 {
                            pruning_table[next_1][next_2] = distance + 1;
                            filled += 1;
                        }
                    }
                }
            }
        }

        distance += 1;
    }

    pruning_table
}
