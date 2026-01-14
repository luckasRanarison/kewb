use crate::constants::{CO_COUNT, CP_COUNT, EO_COUNT, EP_COUNT};

use super::{cubie::CubieCube, index::*};
use rand::{rngs::ThreadRng, prelude::IndexedRandom, rng, Rng};

/// Randomly swaps corner or edges to fix parity.
fn fix_parity(state: &mut CubieCube, rng: &mut ThreadRng, corners: Vec<usize>, edges: Vec<usize>) {
    if rng.random_bool(0.5) {
        swap_edges(state, rng, edges)
    } else {
        swap_corners(state, rng, corners)
    }
}

fn swap_edges(state: &mut CubieCube, rng: &mut ThreadRng, edges: Vec<usize>) {
    let pos: Vec<&usize> = edges.choose_multiple(rng, 2).collect();
    let a = *pos[0];
    let b = *pos[1];
    state.ep.swap(a, b)
}

fn swap_corners(state: &mut CubieCube, rng: &mut ThreadRng, corners: Vec<usize>) {
    let pos: Vec<&usize> = corners.choose_multiple(rng, 2).collect();
    let a = *pos[0];
    let b = *pos[1];
    state.cp.swap(a, b)
}

/// Generates a random state with corners solved.
pub fn generate_state_corners_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        ep: index_to_ep(rng.random_range(0..EP_COUNT)),
        eo: index_to_eo(rng.random_range(0..EO_COUNT)),
        ..Default::default()
    };

    if !state.is_solvable() {
        swap_edges(&mut state, &mut rng, (0..12).collect());
    }

    state
}

/// Generates a random state with edges solved.
pub fn generate_state_edges_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp(rng.random_range(0..CP_COUNT)),
        co: index_to_co(rng.random_range(0..CO_COUNT)),
        ..Default::default()
    };

    if !state.is_solvable() {
        swap_corners(&mut state, &mut rng, (0..8).collect());
    }

    state
}

/// Generates a random state with oriented solved last layer cross.
pub fn generate_state_oll_cross_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp_f2l(rng.random_range(0..4)),
        co: index_to_co_f2l(rng.random_range(0..27)),
        ep: index_to_ep_f2l(rng.random_range(0..24)),
        ..Default::default()
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..4).collect(), (4..8).collect())
    }

    state
}

/// Generates a random state with oriented last layer corners and edges.
pub fn generate_state_oll_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp_f2l(rng.random_range(0..4)),
        ep: index_to_ep_f2l(rng.random_range(0..24)),
        ..Default::default()
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..4).collect(), (4..8).collect())
    }

    state
}

/// Generates a random state with solved First two layer.
pub fn generate_state_f2l_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp_f2l(rng.random_range(0..4)),
        co: index_to_co_f2l(rng.random_range(0..27)),
        ep: index_to_ep_f2l(rng.random_range(0..24)),
        eo: index_to_eo_f2l(rng.random_range(0..8)),
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..4).collect(), (4..8).collect())
    }

    state
}

/// Generates a random state with solved cross.
pub fn generate_state_cross_solved() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp(rng.random_range(0..CP_COUNT)),
        co: index_to_co(rng.random_range(0..CO_COUNT)),
        ep: index_to_ep_cross(rng.random_range(0..40320)),
        eo: index_to_eo_cross(rng.random_range(0..128)),
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..8).collect(), (0..8).collect())
    }

    state
}

/// Generates a random state on the cubie level.
pub fn generate_random_state() -> CubieCube {
    let mut rng = rng();
    let mut state = CubieCube {
        cp: index_to_cp(rng.random_range(0..CP_COUNT)),
        co: index_to_co(rng.random_range(0..CO_COUNT)),
        ep: index_to_ep(rng.random_range(0..EP_COUNT)),
        eo: index_to_eo(rng.random_range(0..EO_COUNT)),
    };

    if !state.is_solvable() {
        fix_parity(&mut state, &mut rng, (0..8).collect(), (0..12).collect())
    }

    state
}
