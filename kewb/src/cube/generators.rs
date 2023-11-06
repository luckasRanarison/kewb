use crate::constants::{CO_COUNT, CP_COUNT, EO_COUNT, EP_COUNT};

use super::{
    cubie::CubieCube,
    index::{index_to_co, index_to_cp, index_to_eo, index_to_ep},
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

/// Randomly swaps corner or edges to fix parity.
fn fix_parity(state: &mut CubieCube, rng: &mut ThreadRng, corners: Vec<usize>, edges: Vec<usize>) {
    if rng.gen_bool(0.5) {
        let pos: Vec<&usize> = edges.choose_multiple(rng, 2).collect();
        let a = *pos[0];
        let b = *pos[1];
        state.ep.swap(a, b)
    } else {
        let pos: Vec<&usize> = corners.choose_multiple(rng, 2).collect();
        let a = *pos[0];
        let b = *pos[1];
        state.cp.swap(a, b)
    }
}

/// Generates a random state on the cubie level.
pub fn generate_random_state() -> CubieCube {
    let mut rng = thread_rng();
    let mut state = CubieCube {
        cp: index_to_cp(rng.gen_range(0..CP_COUNT)),
        co: index_to_co(rng.gen_range(0..CO_COUNT)),
        ep: index_to_ep(rng.gen_range(0..EP_COUNT)),
        eo: index_to_eo(rng.gen_range(0..EO_COUNT)),
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..8).collect(), (0..12).collect())
    }

    state
}
