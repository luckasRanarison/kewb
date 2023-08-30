/// Names the colors of the cube facelets: up, right, face, down, left, back.
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Color {
    U, R, F, D, L, B,
}

impl TryFrom<char> for Color {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Color::U),
            'R' => Ok(Color::R),
            'F' => Ok(Color::F),
            'D' => Ok(Color::D),
            'L' => Ok(Color::L),
            'B' => Ok(Color::B),
            _ => Err("Invalid color value".to_owned()),
        }
    }
}

/// Cube on the facelet level.
/// The facelet representation follows the ordering: U-R-F-D-L-B.
/// A solved facelet is UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB.
pub struct FaceCube {
    pub f: [Color; 54],
}

#[rustfmt::skip]
const SOLVED_FACE_CUBE: FaceCube = FaceCube {
    f: [
        Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U,
        Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R,
        Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F,
        Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D,
        Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L,
        Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B,
    ],
};

impl TryFrom<&str> for FaceCube {
    type Error = String;
    fn try_from(cube_string: &str) -> Result<Self, Self::Error> {
        let mut f: [Color; 54] = SOLVED_FACE_CUBE.f;

        for (i, c) in cube_string.chars().enumerate() {
            f[i] = Color::try_from(c).unwrap();
        }

        Ok(FaceCube { f })
    }
}

/// The names of the facelet positions of the cube.
///
/// ```text
///             |************|
///             |*U1**U2**U3*|
///             |************|
///             |*U4**U5**U6*|
///             |************|
///             |*U7**U8**U9*|
///             |************|
/// ************|************|************|************|
/// *L1**L2**L3*|*F1**F2**F3*|*R1**R2**F3*|*B1**B2**B3*|
/// ************|************|************|************|
/// *L4**L5**L6*|*F4**F5**F6*|*R4**R5**R6*|*B4**B5**B6*|
/// ************|************|************|************|
/// *L7**L8**L9*|*F7**F8**F9*|*R7**R8**R9*|*B7**B8**B9*|
/// ************|************|************|************|
///             |************|
///             |*D1**D2**D3*|
///             |************|
///             |*D4**D5**D6*|
///             |************|
///             |*D7**D8**D9*|
///             |************|
/// ```
#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Facelet {
    U1, U2, U3, U4, _U5, U6, U7, U8, U9,
    R1, R2, R3, R4, _R5, R6, R7, R8, R9,
    F1, F2, F3, F4, _F5, F6, F7, F8, F9,
    D1, D2, D3, D4, _D5, D6, D7, D8, D9,
    L1, L2, L3, L4, _L5, L6, L7, L8, L9,
    B1, B2, B3, B4, _B5, B6, B7, B8, B9,
}

/// Map the corner positions to facelet positions.
pub const CORNER_FACELET: [[Facelet; 3]; 8] = [
    /*UBL=*/ [Facelet::U1, Facelet::L1, Facelet::B3],
    /*UBR=*/ [Facelet::U3, Facelet::B1, Facelet::R3],
    /*UFR=*/ [Facelet::U9, Facelet::R1, Facelet::F3],
    /*UFL=*/ [Facelet::U7, Facelet::F1, Facelet::L3],
    /*DFL=*/ [Facelet::D1, Facelet::L9, Facelet::F7],
    /*DFR=*/ [Facelet::D3, Facelet::F9, Facelet::R7],
    /*DBR=*/ [Facelet::D9, Facelet::R9, Facelet::B7],
    /*DBL=*/ [Facelet::D7, Facelet::B9, Facelet::L7],
];

/// Map the edge positions to facelet positions.
pub const EDGE_FACELET: [[Facelet; 2]; 12] = [
    /*BL=*/ [Facelet::B6, Facelet::L4],
    /*BR=*/ [Facelet::B4, Facelet::R6],
    /*FR=*/ [Facelet::F6, Facelet::R4],
    /*FL=*/ [Facelet::F4, Facelet::L6],
    /*UB=*/ [Facelet::U2, Facelet::B2],
    /*UR=*/ [Facelet::U6, Facelet::R2],
    /*UF=*/ [Facelet::U8, Facelet::F2],
    /*UL=*/ [Facelet::U4, Facelet::L2],
    /*DF=*/ [Facelet::D2, Facelet::F8],
    /*DR=*/ [Facelet::D6, Facelet::R8],
    /*DB=*/ [Facelet::D8, Facelet::B8],
    /*DL=*/ [Facelet::D4, Facelet::L8],
];

/// Map the corner positions to facelet colors.
pub const CORNER_COLOR: [[Color; 3]; 8] = [
    /*UBL=*/ [Color::U, Color::L, Color::B],
    /*UBR=*/ [Color::U, Color::B, Color::R],
    /*UFR=*/ [Color::U, Color::R, Color::F],
    /*UFL=*/ [Color::U, Color::F, Color::L],
    /*DFL=*/ [Color::D, Color::L, Color::F],
    /*DFR=*/ [Color::D, Color::F, Color::R],
    /*DBR=*/ [Color::D, Color::R, Color::B],
    /*DBL=*/ [Color::D, Color::B, Color::L],
];

/// Map the edge positions to facelet colors.
pub const EDGE_COLOR: [[Color; 2]; 12] = [
    /*BL=*/ [Color::B, Color::L],
    /*BR=*/ [Color::B, Color::R],
    /*FR=*/ [Color::F, Color::R],
    /*FL=*/ [Color::F, Color::L],
    /*UB=*/ [Color::U, Color::B],
    /*UR=*/ [Color::U, Color::R],
    /*UF=*/ [Color::U, Color::F],
    /*UL=*/ [Color::U, Color::L],
    /*DF=*/ [Color::D, Color::F],
    /*DR=*/ [Color::D, Color::R],
    /*DB=*/ [Color::D, Color::B],
    /*DL=*/ [Color::D, Color::L],
];

#[cfg(test)]
mod test {
    use super::*;
    use crate::cube::state::Corner::*;
    use crate::cube::state::Edge::*;

    #[test]
    fn test_facelet_to_cubie() {
        // One scramble that produces these faces:
        // F L' B R' U R U B' L2 R' F2 U2 L' F2 D F U R' D R U' L' R2 D2
        let faces = "DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL";
        let face_cube = FaceCube::try_from(faces).unwrap();

        let actual_state = crate::cube::state::State::try_from(&face_cube).unwrap();
        assert_eq!(
            actual_state,
            crate::cube::state::State {
                cp: [DFL, UBL, DBR, UFR, UBR, DFR, UFL, DBL],
                co: [0, 1, 0, 2, 0, 1, 0, 2],
                ep: [DF, DB, DR, UF, FR, UB, UL, DL, UR, FL, BR, BL],
                eo: [1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0]
            }
        );
        // One list of moves that solves this state:
        // L2 B' D R F B' L U B R' U' B2 D L2 D2 R2 B2 D' B2 D F2 U
    }
}
