use std::str::FromStr;

use crate::{error::Error, CubieCube, Move, Solver};

pub fn scramble_from_str(s: &str) -> Result<Vec<Move>, Error> {
    s.split_whitespace()
        .map(|word| Move::from_str(word.trim()))
        .collect()
}

pub fn scramble_from_state(state: CubieCube, solver: &mut Solver) -> Result<Vec<Move>, Error> {
    let solution = solver.solve(state);

    if let Some(solution) = solution {
        Ok(solution
            .get_all_moves()
            .iter()
            .rev()
            .map(|m| m.get_inverse())
            .collect())
    } else {
        Err(Error::InvalidCubieValue)
    }
}
