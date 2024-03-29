use std::{fmt, str::FromStr};

use crate::error::Error;

use self::Move::*;
use super::cubie::{Corner::*, CubieCube, Edge::*};

/// Layer moves, Up, Down, Right, Left, Face, Back.
/// $ clockwise, $2 double, $3 counter-clockwise.
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    U, U2, U3,
    D, D2, D3,
    R, R2, R3,
    L, L2, L3,
    F, F2, F3,
    B, B2, B3,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            U3 => write!(f, "U'"),
            D3 => write!(f, "D'"),
            R3 => write!(f, "R'"),
            L3 => write!(f, "L'"),
            F3 => write!(f, "F'"),
            B3 => write!(f, "B'"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(R),
            "R'" => Ok(R3),
            "R2" => Ok(R2),
            "L" => Ok(L),
            "L'" => Ok(L3),
            "L2" => Ok(L2),
            "U" => Ok(U),
            "U'" => Ok(U3),
            "U2" => Ok(U2),
            "D" => Ok(D),
            "D'" => Ok(D3),
            "D2" => Ok(D2),
            "F" => Ok(F),
            "F'" => Ok(F3),
            "F2" => Ok(F2),
            "B" => Ok(B),
            "B'" => Ok(B3),
            "B2" => Ok(B2),
            _ => Err(Error::InvalidScramble),
        }
    }
}

#[rustfmt::skip]
impl Move {
    pub fn is_inverse(&self, other: Move) -> bool {
        matches!(
            (&self, other),
            (U | U2 | U3, D | D2 | D3) 
            | (R | R2 | R3, L | L2 | L3) 
            | (F | F2 | F3, B | B2 | B3),
        )
    }

    pub fn is_same_layer(&self, other: Move) -> bool {
        matches!(
            (&self, other),
            (U | U2 | U3, U | U2 | U3)
            | (D | D2 | D3, D | D2 | D3)
            | (R | R2 | R3, R | R2 | R3)
            | (L | L2 | L3, L | L2 | L3)
            | (F | F2 | F3, F | F2 | F3)
            | (B | B2 | B3, B | B2 | B3)
        )
    }

    pub fn get_inverse(self) -> Self {
        match self {
            U => U3,
            U3 => U,
            D => D3,
            D3 => D,
            R => R3,
            R3 => R,
            L => L3,
            L3 => L,
            F => F3,
            F3 => F,
            B => B3,
            B3 => B,
            _ => self,
        }
    }
}

pub fn is_move_available(prev: Move, current: Move) -> bool {
    current != prev && !current.is_inverse(prev) && !current.is_same_layer(prev)
}

pub const U_MOVE: CubieCube = CubieCube {
    cp: [UFL, UBL, UBR, UFR, DFL, DFR, DBR, DBL],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [BL, BR, FR, FL, UL, UB, UR, UF, DF, DR, DB, DL],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const D_MOVE: CubieCube = CubieCube {
    cp: [UBL, UBR, UFR, UFL, DBL, DFL, DFR, DBR],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [BL, BR, FR, FL, UB, UR, UF, UL, DL, DF, DR, DB],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const R_MOVE: CubieCube = CubieCube {
    cp: [UBL, UFR, DFR, UFL, DFL, DBR, UBR, DBL],
    co: [0, 1, 2, 0, 0, 1, 2, 0],
    ep: [BL, UR, DR, FL, UB, FR, UF, UL, DF, BR, DB, DL],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const L_MOVE: CubieCube = CubieCube {
    cp: [DBL, UBR, UFR, UBL, UFL, DFR, DBR, DFL],
    co: [2, 0, 0, 1, 2, 0, 0, 1],
    ep: [DL, BR, FR, UL, UB, UR, UF, BL, DF, DR, DB, FL],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const F_MOVE: CubieCube = CubieCube {
    cp: [UBL, UBR, UFL, DFL, DFR, UFR, DBR, DBL],
    co: [0, 0, 1, 2, 1, 2, 0, 0],
    ep: [BL, BR, UF, DF, UB, UR, FL, UL, FR, DR, DB, DL],
    eo: [0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0],
};

pub const B_MOVE: CubieCube = CubieCube {
    cp: [UBR, DBR, UFR, UFL, DFL, DFR, DBL, UBL],
    co: [1, 2, 0, 0, 0, 0, 1, 2],
    ep: [UB, DB, FR, FL, BR, UR, UF, UL, DF, DR, BL, DL],
    eo: [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0],
};
