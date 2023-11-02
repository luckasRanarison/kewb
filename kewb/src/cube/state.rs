use crate::error::Error;

use self::{Corner::*, Edge::*, Move::*};
use super::{facelet::*, moves::*};
use std::ops::Mul;

/// Represents the 8 corners on the cube, described by the layer they are on.
/// Example: UBL (Up, Bottom, Left).
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Corner {
    UBL, UBR, UFR, UFL,
    DFL, DFR, DBR, DBL,
}

impl TryFrom<u8> for Corner {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UBL),
            1 => Ok(UBR),
            2 => Ok(UFR),
            3 => Ok(UFL),
            4 => Ok(DFL),
            5 => Ok(DFR),
            6 => Ok(DBR),
            7 => Ok(DBL),
            _ => Err(Error::InvalidCorner),
        }
    }
}

/// Represents the 12 edges on the cube, described by the layer they are on.
/// Example: BL (Bottom, Left).
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Edge {
    BL, BR, FR, FL,
    UB, UR, UF, UL,
    DF, DR, DB, DL,
}

impl TryFrom<u8> for Edge {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BL),
            1 => Ok(BR),
            2 => Ok(FR),
            3 => Ok(FL),
            4 => Ok(UB),
            5 => Ok(UR),
            6 => Ok(UF),
            7 => Ok(UL),
            8 => Ok(DF),
            9 => Ok(DR),
            10 => Ok(DB),
            11 => Ok(DL),
            _ => Err(Error::InvalidEdge),
        }
    }
}

/// Cube on the cubie level.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct State {
    /// Corner permutation, relative to SOLVED_STATE.
    pub cp: [Corner; 8],
    /// Corner orientation, 3 possible values: 0 (correctly oriented), 1 (twisted clockwise), 2 (twisted counter-clockwise).
    pub co: [u8; 8],
    /// Edge permutation, relative to SOLVED_STATE.
    pub ep: [Edge; 12],
    /// Edge orientation, 2 possible values: 0 (correctly oriented), 1 (flipped).
    pub eo: [u8; 12],
}

impl Mul for State {
    type Output = Self;

    fn mul(self, rhs: State) -> Self::Output {
        let mut res = SOLVED_STATE;
        // (A * B).c = A(B(x).c).c
        // (A * B).o = A(B(x).c).o + B(x).o

        for i in 0..8 {
            res.cp[i] = self.cp[rhs.cp[i] as usize];
            res.co[i] = (self.co[rhs.cp[i] as usize] + rhs.co[i]) % 3;
        }

        for i in 0..12 {
            res.ep[i] = self.ep[rhs.ep[i] as usize];
            res.eo[i] = (self.eo[rhs.ep[i] as usize] + rhs.eo[i]) % 2;
        }

        res
    }
}

impl State {
    pub fn apply_move(self, move_name: Move) -> Self {
        let move_state = match move_name {
            U => U_MOVE,
            U2 => U_MOVE * U_MOVE,
            U3 => U_MOVE * U_MOVE * U_MOVE,
            D => D_MOVE,
            D2 => D_MOVE * D_MOVE,
            D3 => D_MOVE * D_MOVE * D_MOVE,
            R => R_MOVE,
            R2 => R_MOVE * R_MOVE,
            R3 => R_MOVE * R_MOVE * R_MOVE,
            L => L_MOVE,
            L2 => L_MOVE * L_MOVE,
            L3 => L_MOVE * L_MOVE * L_MOVE,
            F => F_MOVE,
            F2 => F_MOVE * F_MOVE,
            F3 => F_MOVE * F_MOVE * F_MOVE,
            B => B_MOVE,
            B2 => B_MOVE * B_MOVE,
            B3 => B_MOVE * B_MOVE * B_MOVE,
        };

        self * move_state
    }

    pub fn apply_moves(&self, moves: &Vec<Move>) -> Self {
        let mut state = *self;

        for m in moves {
            state = state.apply_move(*m);
        }

        state
    }

    /// Returns the number of corner permutations needed to solve the corners.
    pub fn count_corner_perm(&self) -> u8 {
        let mut count = 0;
        let mut cp = self.cp;

        for i in 0..8 {
            if cp[i] as usize != i {
                if let Some(j) = (i + 1..8).find(|&j| cp[j] as usize == i) {
                    cp.swap(i, j);
                    count += 1;
                }
            }
        }

        count
    }

    /// Returns the number of edge permutations needed to solve the edges.
    pub fn count_edge_perm(&self) -> u8 {
        let mut count = 0;
        let mut ep = self.ep;

        for i in 0..12 {
            if ep[i] as usize != i {
                if let Some(j) = (i + 1..12).find(|&j| ep[j] as usize == i) {
                    ep.swap(i, j);
                    count += 1;
                }
            }
        }

        count
    }
}

impl From<&Vec<Move>> for State {
    fn from(moves: &Vec<Move>) -> Self {
        let mut state = SOLVED_STATE;

        for m in moves {
            state = state.apply_move(*m);
        }

        state
    }
}

/// Gives State (cubie) representation of a face cube (facelet).
impl TryFrom<&FaceCube> for State {
    type Error = Error;
    fn try_from(face_cube: &FaceCube) -> Result<Self, Self::Error> {
        let mut ori: usize = 0;
        let mut state = SOLVED_STATE;
        let mut col1;
        let mut col2;

        for i in 0..8 {
            let i = Corner::try_from(i)?;
            // get the colors of the cubie at corner i, starting with U/D
            for index in 0..3 {
                ori = index;
                if face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::U
                    || face_cube.f[CORNER_FACELET[i as usize][ori] as usize] == Color::D
                {
                    break;
                }
            }

            col1 = face_cube.f[CORNER_FACELET[i as usize][(ori + 1) % 3] as usize];
            col2 = face_cube.f[CORNER_FACELET[i as usize][(ori + 2) % 3] as usize];

            for j in 0..8 {
                let j = Corner::try_from(j)?;
                if col1 == CORNER_COLOR[j as usize][1] && col2 == CORNER_COLOR[j as usize][2] {
                    // in cornerposition i we have cornercubie j
                    state.cp[i as usize] = j;
                    state.co[i as usize] = ori as u8 % 3;
                    break;
                }
            }
        }

        for i in 0..12 {
            let i = Edge::try_from(i)?;
            for j in 0..12 {
                let j = Edge::try_from(j)?;
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][0]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][1]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 0;
                    break;
                }
                if face_cube.f[EDGE_FACELET[i as usize][0] as usize] == EDGE_COLOR[j as usize][1]
                    && face_cube.f[EDGE_FACELET[i as usize][1] as usize]
                        == EDGE_COLOR[j as usize][0]
                {
                    state.ep[i as usize] = j;
                    state.eo[i as usize] = 1;
                    break;
                }
            }
        }

        let c_perm = state.count_corner_perm();
        let e_perm = state.count_edge_perm();

        if c_perm % 2 != e_perm % 2 {
            Err(Error::InvalidFaceletValue)
        } else {
            Ok(state)
        }
    }
}

pub const SOLVED_STATE: State = State {
    cp: [UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [BL, BR, FR, FL, UB, UR, UF, UL, DF, DR, DB, DL],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

#[cfg(test)]
mod test {
    use super::{Corner::*, Edge::*, Move::*, SOLVED_STATE};
    use crate::cube::{moves::*, state::State};

    #[test]
    fn test_mult() {
        let state = SOLVED_STATE.apply_move(R);
        assert_eq!(state, R_MOVE);

        let r2_state = SOLVED_STATE.apply_move(R).apply_move(R);
        assert_eq!(r2_state, R_MOVE * R_MOVE);

        let r3_state = r2_state.apply_move(R);
        assert_eq!(r3_state, r2_state * R_MOVE);

        let fr_state = State {
            cp: [UBL, UFL, UFR, DFL, DFR, DBR, UBR, DBL],
            co: [0, 2, 1, 2, 1, 1, 2, 0],
            ep: [BL, UR, DR, DF, UB, UF, FL, UL, FR, BR, DB, DL],
            eo: [0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0],
        };

        assert_eq!(F_MOVE * R_MOVE, fr_state);
    }

    #[test]
    fn test_move_sequence() {
        // (R U R' U') * 6
        let moves = vec![
            R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3,
        ];
        let state = SOLVED_STATE.apply_moves(&moves);

        assert_eq!(state, SOLVED_STATE);
    }

    #[test]
    fn test_scramble() {
        // U F' D' F2 D B2 D' R2 U' F2 R2 D2 R2 U' L B L R F' D B'
        let scramble = vec![
            U, F3, D3, F2, D, B2, D3, R2, U3, F2, R2, D2, R2, U3, L, B, L, R, F3, D, B3,
        ];
        let state = SOLVED_STATE.apply_moves(&scramble);

        let expected = State {
            cp: [DFL, UBL, DFR, UBR, UFL, DBR, DBL, UFR],
            co: [1, 2, 2, 0, 0, 0, 2, 2],
            ep: [UF, UR, DL, DB, BL, DF, UB, FL, UL, BR, FR, DR],
            eo: [0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        };

        assert_eq!(state, expected);
    }

    #[test]
    fn test_perm_count() {
        let state = SOLVED_STATE;

        assert_eq!(state.count_corner_perm(), 0);
        assert_eq!(state.count_edge_perm(), 0);

        let state = State::from(&vec![R, U, R3, U3]);

        assert_eq!(state.count_corner_perm(), 2);
        assert_eq!(state.count_edge_perm(), 2);
        let state = State::from(&vec![
            R, U3, R3, U3, R, U, R, D, R3, U3, R, D3, R3, U2, R3, U3,
        ]);

        assert_eq!(state.count_corner_perm(), 1);
        assert_eq!(state.count_edge_perm(), 1);
    }
}