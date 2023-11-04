use crate::cube::moves::Move;

use super::{moves::*, utils::*};
use bincode::{Decode, Encode};

/// Collection of pruning table for filtering unsolvable state at a given depth.
#[derive(Encode, Decode)]
pub struct PruningTable {
    pub co_e: Table<u8>,
    pub eo_e: Table<u8>,
    pub cp_e: Table<u8>,
    pub ep_e: Table<u8>,
}

impl Default for PruningTable {
    fn default() -> Self {
        Self {
            co_e: get_prune_table(get_co_table(), get_e_combo_table(), &ALL_MOVES),
            eo_e: get_prune_table(get_eo_table(), get_e_combo_table(), &ALL_MOVES),
            cp_e: get_prune_table(get_cp_table(), get_e_ep_table(), &PHASE2_MOVES),
            ep_e: get_prune_table(get_ud_ep_table(), get_e_ep_table(), &PHASE2_MOVES),
        }
    }
}

pub fn get_prune_table(table1: Table<u16>, table2: Table<u16>, moves: &[Move]) -> Table<u8> {
    let len1 = table1.len();
    let len2 = table2.len();
    let fill_size = len1 * len2;
    let mut pruning_table = vec![vec![u8::MAX; len2]; len1];
    let mut distance = 0;
    let mut filled: usize = 1;

    pruning_table[0][0] = 0;

    while filled != fill_size {
        for (i, ti) in table1.iter().enumerate() {
            for (j, tj) in table2.iter().enumerate() {
                if pruning_table[i][j] == distance {
                    for m in 0..moves.len() {
                        let k = ti[m] as usize;
                        let l = tj[m] as usize;

                        if pruning_table[k][l] == u8::MAX {
                            pruning_table[k][l] = distance + 1;
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
