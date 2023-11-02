use super::{
    index::{index_to_co, index_to_cp, index_to_eo, index_to_ep},
    state::State,
};
use rand::{thread_rng, Rng};

pub fn generate_random_state() -> State {
    let mut rng = thread_rng();
    let mut state = State {
        cp: index_to_cp(rng.gen_range(0..40320)),
        co: index_to_co(rng.gen_range(0..2187)),
        ep: index_to_ep(rng.gen_range(0..479001599)),
        eo: index_to_eo(rng.gen_range(0..2048)),
    };

    let c_perm = state.count_corner_perm();
    let e_perm = state.count_edge_perm();
    let is_even = |a| a % 2 == 0;

    if !is_even(c_perm) && is_even(e_perm) {
        state.cp.swap(0, 1);
    } else if !is_even(e_perm) && is_even(c_perm) {
        state.ep.swap(0, 1);
    }

    state
}
