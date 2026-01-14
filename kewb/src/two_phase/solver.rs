use std::{fmt, time::Duration};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use wasmtimer::std::Instant;

use crate::cube::{
    cubie::CubieCube,
    index::*,
    moves::{is_move_available, Move},
};

use super::pruning::PruningTable;
use super::utils::{ALL_MOVES, PHASE2_MOVES};
use super::{moves::MoveTable, utils::DataTable};

trait Phase {
    fn is_solved(&self) -> bool;
    fn next(&self, table: &MoveTable, move_index: usize) -> Self;
    fn prune(&self, table: &PruningTable, depth: u8) -> bool;
}

#[derive(Debug)]
struct Phase1State {
    co_index: usize,
    eo_index: usize,
    e_combo_index: usize,
}

impl Phase for Phase1State {
    fn is_solved(&self) -> bool {
        self.co_index == 0 && self.eo_index == 0 && self.e_combo_index == 0
    }

    fn next(&self, table: &MoveTable, move_index: usize) -> Self {
        let co_index = table.co[self.co_index][move_index].into();
        let eo_index = table.eo[self.eo_index][move_index].into();
        let e_combo_index = table.e_combo[self.e_combo_index][move_index].into();

        Self {
            co_index,
            eo_index,
            e_combo_index,
        }
    }

    fn prune(&self, table: &PruningTable, depth: u8) -> bool {
        let co_e_dist = table.co_e[self.co_index][self.e_combo_index];
        let eo_e_dist = table.eo_e[self.eo_index][self.e_combo_index];
        let max = co_e_dist.max(eo_e_dist);

        max > depth
    }
}

impl From<CubieCube> for Phase1State {
    fn from(value: CubieCube) -> Self {
        let co_index = co_to_index(&value.co).into();
        let eo_index = eo_to_index(&value.eo).into();
        let e_combo_index = e_combo_to_index(&value.ep).into();

        Self {
            co_index,
            eo_index,
            e_combo_index,
        }
    }
}

struct Phase2State {
    cp_index: usize,
    ep_index: usize,
    e_ep_index: usize,
}

impl From<CubieCube> for Phase2State {
    fn from(value: CubieCube) -> Self {
        let cp_index = cp_to_index(&value.cp).into();
        let ep_index = ud_ep_to_index(&value.ep).into();
        let e_ep_index = e_ep_to_index(&value.ep).into();

        Self {
            cp_index,
            ep_index,
            e_ep_index,
        }
    }
}

impl Phase for Phase2State {
    fn is_solved(&self) -> bool {
        self.cp_index == 0 && self.ep_index == 0 && self.e_ep_index == 0
    }

    fn next(&self, table: &MoveTable, move_index: usize) -> Self {
        let cp_index = table.cp[self.cp_index][move_index].into();
        let ep_index = table.ep[self.ep_index][move_index].into();
        let e_ep_index = table.e_ep[self.e_ep_index][move_index].into();

        Self {
            cp_index,
            ep_index,
            e_ep_index,
        }
    }

    fn prune(&self, table: &PruningTable, depth: u8) -> bool {
        let cp_e_dist = table.cp_e[self.cp_index][self.e_ep_index];
        let ep_e_dist = table.ep_e[self.ep_index][self.e_ep_index];
        let max = cp_e_dist.max(ep_e_dist);

        max > depth
    }
}

/// Two phase solution.
#[derive(Debug, Clone)]
pub struct Solution {
    pub phase1: Vec<Move>,
    pub phase2: Vec<Move>,
}

impl Solution {
    pub fn len(&self) -> usize {
        self.phase1.len() + self.phase2.len()
    }

    pub fn is_empty(&self) -> bool {
        self.phase1.is_empty() && self.phase2.is_empty()
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut clone = self.phase1.clone();
        clone.extend(&self.phase2);
        let stringified = clone
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{stringified}")
    }
}

impl Solution {
    pub fn phase1_to_string(&self) -> String {
        self.phase1
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn phase2_to_string(&self) -> String {
        self.phase2
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut solution = self.phase1.clone();
        solution.extend(&self.phase2);
        solution
    }
}

/// Two phase solver.
pub struct Solver<'a> {
    data_table: &'a DataTable,
    max_length: u8,
    timeout: Option<Duration>,
    initial_state: CubieCube,
    solution_phase1: Vec<Move>,
    solution_phase2: Vec<Move>,
    best_solution: Option<Solution>,
}

impl<'a> Solver<'a> {
    pub fn new(data_table: &'a DataTable, max_length: u8, timeout: Option<f32>) -> Self {
        let timeout = timeout.map(Duration::from_secs_f32);

        Self {
            data_table,
            initial_state: CubieCube::default(),
            max_length,
            timeout,
            solution_phase1: vec![],
            solution_phase2: vec![],
            best_solution: None,
        }
    }

    /// Resets the solver state.
    pub fn clear(&mut self) {
        self.initial_state = CubieCube::default();
        self.solution_phase1.clear();
        self.solution_phase2.clear();
        self.best_solution.take();
    }

    /// Solves the cube using the two phase algorithm.
    pub fn solve(&mut self, state: CubieCube) -> Option<Solution> {
        self.initial_state = state;

        let start = Instant::now();

        for depth in 0..=self.max_length {
            let state = Phase1State::from(state);
            let found = self.solve_phase1(state, depth, start);

            if let Some(timeout) = self.timeout {
                if start.elapsed() > timeout {
                    return self.best_solution.clone();
                }
            } else if found {
                return self.best_solution.clone();
            }
        }

        None
    }

    fn solve_phase1(&mut self, state: Phase1State, depth: u8, time: Instant) -> bool {
        if let Some(timeout) = self.timeout {
            if time.elapsed() > timeout {
                return true;
            }
        }

        if depth == 0 && state.is_solved() {
            let mut cube_state = self.initial_state;

            for m in &self.solution_phase1 {
                cube_state = cube_state.apply_move(*m);
            }

            let max_depth = match self.solution_phase1.len() {
                0 => self.max_length,
                _ => {
                    if self.max_length > self.solution_phase1.len() as u8 {
                        self.max_length - self.solution_phase1.len() as u8
                    } else {
                        return true;
                    }
                }
            };

            for phase2_depth in 0..max_depth {
                let state = Phase2State::from(cube_state);
                if self.solve_phase2(state, phase2_depth, time) {
                    return true;
                }
            }

            return false;
        }

        if state.prune(&self.data_table.pruning_table, depth) || depth == 0 {
            return false;
        }

        for (i, m) in ALL_MOVES.iter().enumerate() {
            if let Some(prev) = self.solution_phase1.last() {
                if !is_move_available(*prev, *m) {
                    continue;
                }
            }

            self.solution_phase1.push(*m);

            let new_state = state.next(&self.data_table.move_table, i);
            let found = self.solve_phase1(new_state, depth - 1, time);

            if found {
                return true;
            }

            self.solution_phase1.pop();
        }

        false
    }

    fn solve_phase2(&mut self, state: Phase2State, depth: u8, time: Instant) -> bool {
        if let Some(timeout) = self.timeout {
            if time.elapsed() > timeout {
                return true;
            }
        }

        if depth == 0 && state.is_solved() {
            let solution = Solution {
                phase1: self.solution_phase1.clone(),
                phase2: self.solution_phase2.clone(),
            };

            if let Some(best_solution) = &mut self.best_solution {
                let current_length = self.solution_phase1.len() + self.solution_phase2.len();
                if best_solution.len() > current_length {
                    *best_solution = solution
                }
            } else {
                self.best_solution = Some(solution)
            }

            return true;
        }

        if state.prune(&self.data_table.pruning_table, depth) || depth == 0 {
            return false;
        }

        for (i, m) in PHASE2_MOVES.iter().enumerate() {
            if let Some(prev) = self.solution_phase2.last() {
                if !is_move_available(*prev, *m) {
                    continue;
                }
            } else if let Some(prev) = self.solution_phase1.last() {
                if !is_move_available(*prev, *m) {
                    continue;
                }
            }

            self.solution_phase2.push(*m);

            let new_state = state.next(&self.data_table.move_table, i);
            let found = self.solve_phase2(new_state, depth - 1, time);

            if found {
                return true;
            }

            self.solution_phase2.pop();
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{cube::cubie::SOLVED_CUBIE_CUBE, Move::*};

    #[test]
    fn test_solve() {
        let scramble = vec![
            D3, R2, L3, U2, F, R, F3, D2, R2, F2, B2, U2, R2, F2, U, R2, U3, R2, D2,
        ];
        let state = CubieCube::from(&scramble);
        let table = DataTable::default();
        let mut solver = Solver::new(&table, 23, None);
        let solution = solver.solve(state);
        let solved_state = state.apply_moves(&solution.unwrap().get_all_moves());

        assert_eq!(solved_state, SOLVED_CUBIE_CUBE);
    }
}
