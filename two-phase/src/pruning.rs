use bincode::{Decode, Encode};
use cube::moves::Move;

use crate::utils::*;

#[derive(Encode, Decode)]
pub struct PruningTable {
    pub co_e: Table<u8>,
    pub eo_e: Table<u8>,
    pub cp_e: Table<u8>,
    pub ep_e: Table<u8>,
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
