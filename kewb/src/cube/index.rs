use super::cubie::{Corner, Edge};

fn fill_orientation_slice(slice: &mut [u8], cases: u8, index: u16) {
    let len = slice.len();
    let mut index = index;
    let mut orientation_sum = 0;

    for i in (0..len - 1).rev() {
        slice[i] = (index % cases as u16) as u8;
        index /= cases as u16;
        orientation_sum += slice[i];
    }

    slice[len - 1] = (cases - orientation_sum % cases) % cases;
}

fn fill_perm_slice(slice: &mut [u8], index: usize) {
    let len = slice.len();
    let mut index = index;
    let mut perm = vec![0; len];

    for i in (0..(len - 1)).rev() {
        perm[i] = (index % (len - i)) as u8;
        index /= len - i;
        for j in (i + 1)..len {
            if perm[j] >= perm[i] {
                perm[j] += 1;
            }
        }
    }

    for i in 0..len {
        slice[i] += perm[i];
    }
}

pub fn slice_to_index(cp: &[u8]) -> usize {
    let len = cp.len();
    let mut index = 0;

    for i in 0..len {
        index *= len - i;
        for j in i + 1..len {
            if cp[i] > cp[j] {
                index += 1;
            }
        }
    }

    index
}

pub fn co_to_index(corner: &[u8; 8]) -> u16 {
    let mut index = 0;

    for co in &corner[0..7] {
        index = index * 3 + *co as u16;
    }

    index
}

pub fn index_to_co(index: u16) -> [u8; 8] {
    let mut co = [0; 8];

    fill_orientation_slice(&mut co, 3, index);
    co
}

pub fn eo_to_index(edge: &[u8; 12]) -> u16 {
    let mut index = 0;

    for eo in &edge[0..11] {
        index = index * 2 + *eo as u16;
    }

    index
}

pub fn index_to_eo(index: u16) -> [u8; 12] {
    let mut eo = [0; 12];

    fill_orientation_slice(&mut eo, 2, index);
    eo
}

fn calculate_combo(n: u8, k: u8) -> u16 {
    if k > n {
        return 0;
    }

    let mut result: u16 = 1;
    for i in 0..k as u16 {
        result *= n as u16 - i;
        result /= i + 1;
    }

    result
}

pub fn e_combo_to_index(edge: &[Edge; 12]) -> u16 {
    let mut index = 0;
    let mut k = 4;

    for i in (0..12).rev() {
        if edge[i] as u8 <= 3 {
            index += calculate_combo(i as u8, k);
            k -= 1;
        }
    }

    index
}

pub fn index_to_e_combo(mut index: u16) -> [Edge; 12] {
    let mut combo: [u8; 12] = [4; 12]; // fake ep
    let mut k = 4;

    for i in (0..12).rev() {
        if index >= calculate_combo(i, k) {
            combo[i as usize] = k - 1;
            index -= calculate_combo(i, k);
            k -= 1;
        }
    }

    combo.map(|value| Edge::try_from(value).unwrap())
}

pub fn cp_to_index(cp: &[Corner; 8]) -> u16 {
    let slice = cp.map(|c| c as u8);
    slice_to_index(&slice) as u16
}

pub fn index_to_cp(index: u16) -> [Corner; 8] {
    let mut cp: [u8; 8] = [0; 8];

    fill_perm_slice(&mut cp, index as usize);
    cp.map(|value| Corner::try_from(value).unwrap())
}

pub fn ep_to_index(ep: &[Edge; 12]) -> u32 {
    let slice = ep.map(|e| e as u8);
    slice_to_index(&slice) as u32
}

pub fn index_to_ep(index: u32) -> [Edge; 12] {
    let mut ep: [u8; 12] = [0; 12];

    fill_perm_slice(&mut ep, index as usize);
    ep.map(|value| Edge::try_from(value).unwrap())
}

pub fn ud_ep_to_index(ep: &[Edge; 12]) -> u16 {
    let slice = ep[4..12].iter().map(|&e| e as u8).collect::<Vec<_>>();
    slice_to_index(&slice) as u16
}

pub fn index_to_ud_ep(index: u16) -> [Edge; 12] {
    let mut ep = [4; 12]; // fake ep

    fill_perm_slice(&mut ep[4..12], index as usize);
    ep.map(|value| Edge::try_from(value).unwrap())
}

pub fn e_ep_to_index(ep: &[Edge; 12]) -> u16 {
    let slice = ep[..4].iter().map(|&e| e as u8).collect::<Vec<_>>();
    slice_to_index(&slice) as u16
}

pub fn index_to_e_ep(index: u16) -> [Edge; 12] {
    let mut ep = [0; 12]; // fake ep

    fill_perm_slice(&mut ep[..4], index as usize);
    ep.map(|value| Edge::try_from(value).unwrap())
}

pub fn index_to_ep_cross(index: u16) -> [Edge; 12] {
    let mut ep = [0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11]; // fake ep

    fill_perm_slice(&mut ep[..8], index as usize);
    ep.map(|value| Edge::try_from(value).unwrap())
}

pub fn index_to_eo_cross(index: u16) -> [u8; 12] {
    let mut eo = [0; 12];

    fill_orientation_slice(&mut eo[..8], 2, index);
    eo
}

pub fn index_to_cp_f2l(index: u16) -> [Corner; 8] {
    let mut cp: [u8; 8] = [0, 0, 0, 0, 4, 5, 6, 7];

    fill_perm_slice(&mut cp[..4], index as usize);
    cp.map(|value| Corner::try_from(value).unwrap())
}

pub fn index_to_co_f2l(index: u16) -> [u8; 8] {
    let mut co: [u8; 8] = [0; 8];

    fill_orientation_slice(&mut co[..4], 3, index);
    co
}

pub fn index_to_ep_f2l(index: u16) -> [Edge; 12] {
    let mut ep = [0, 1, 2, 3, 4, 4, 4, 4, 8, 9, 10, 11]; // fake ep

    fill_perm_slice(&mut ep[4..8], index as usize);
    ep.map(|value| Edge::try_from(value).unwrap())
}

pub fn index_to_eo_f2l(index: u16) -> [u8; 12] {
    let mut eo = [0; 12];

    fill_orientation_slice(&mut eo[4..8], 2, index);
    eo
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cube::cubie::{Edge::*, SOLVED_CUBIE_CUBE};

    #[test]
    fn test_co_to_index() {
        assert_eq!(co_to_index(&SOLVED_CUBIE_CUBE.co), 0);
        assert_eq!(index_to_co(0), SOLVED_CUBIE_CUBE.co);

        let co = [2, 0, 0, 1, 1, 0, 0, 2];
        assert_eq!(co_to_index(&co), 1494);
        assert_eq!(index_to_co(1494), co);
    }

    #[test]
    fn test_eo() {
        assert_eq!(eo_to_index(&SOLVED_CUBIE_CUBE.eo), 0);
        assert_eq!(index_to_eo(0), SOLVED_CUBIE_CUBE.eo);

        let eo = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(eo_to_index(&eo), 2047);
        assert_eq!(index_to_eo(2047), eo);
    }

    #[test]
    fn test_e_combo() {
        assert_eq!(e_combo_to_index(&SOLVED_CUBIE_CUBE.ep), 0);
        let fake_combo = [BL, BR, FR, FL, UB, UB, UB, UB, UB, UB, UB, UB];
        assert_eq!(index_to_e_combo(0), fake_combo);

        let fake_combo = [UB, UB, UB, UB, UB, UB, UB, UB, BL, BR, FR, FL];
        assert_eq!(e_combo_to_index(&fake_combo), 494);
        assert_eq!(index_to_e_combo(494), fake_combo);
    }

    #[test]
    fn test_cp() {
        assert_eq!(cp_to_index(&SOLVED_CUBIE_CUBE.cp), 0);
        assert_eq!(index_to_cp(0), SOLVED_CUBIE_CUBE.cp);

        let mut corners = SOLVED_CUBIE_CUBE.cp;
        corners.reverse();
        assert_eq!(cp_to_index(&corners), 40319);
        assert_eq!(index_to_cp(40319), corners);
    }

    #[test]
    fn test_ud_ep() {
        assert_eq!(ud_ep_to_index(&SOLVED_CUBIE_CUBE.ep), 0);
        let ud_ep = index_to_ud_ep(0);
        assert_eq!(&ud_ep[4..12], &SOLVED_CUBIE_CUBE.ep[4..12]);

        let edges = [BL, BR, FR, FL, DL, DB, DR, DF, UL, UF, UR, UB];
        assert_eq!(ud_ep_to_index(&edges), 40319);
        let ud_ep = index_to_ud_ep(40319);
        assert_eq!(&ud_ep[4..12], &edges[4..12]);
    }

    #[test]
    fn test_e_ep() {
        assert_eq!(e_ep_to_index(&SOLVED_CUBIE_CUBE.ep), 0);
        let e_ep = index_to_e_ep(0);
        assert_eq!(&e_ep[..4], &SOLVED_CUBIE_CUBE.ep[..4]);

        let edges = [FL, FR, BR, BL, UB, UR, UF, UL, DF, DR, DB, DL];
        assert_eq!(e_ep_to_index(&edges), 23);
        let e_ep = index_to_e_ep(23);
        assert_eq!(&e_ep[..4], &edges[..4]);
    }

    #[test]
    fn test_ep() {
        assert_eq!(ep_to_index(&SOLVED_CUBIE_CUBE.ep), 0);
        assert_eq!(index_to_ep(0), SOLVED_CUBIE_CUBE.ep);

        let mut edges = SOLVED_CUBIE_CUBE.ep;
        edges.reverse();
        assert_eq!(ep_to_index(&edges), 479001599);
        assert_eq!(index_to_ep(479001599), edges);
    }
}
