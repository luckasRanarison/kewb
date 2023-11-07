use crate::{cube::moves::Move::*, error::Error, CubieCube, Move, Solver};

pub fn scramble_from_str(string: &str) -> Result<Vec<Move>, Error> {
    let mut scramble = vec![];

    for word in string.split_whitespace() {
        let m = match word.trim() {
            "R" => R,
            "R'" => R3,
            "R2" => R2,
            "L" => L,
            "L'" => L3,
            "L2" => L2,
            "U" => U,
            "U'" => U3,
            "U2" => U2,
            "D" => D,
            "D'" => D3,
            "D2" => D2,
            "F" => F,
            "F'" => F3,
            "F2" => F2,
            "B" => B,
            "B'" => B3,
            "B2" => B2,
            _ => return Err(Error::InvalidScramble),
        };

        scramble.push(m);
    }

    Ok(scramble)
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
