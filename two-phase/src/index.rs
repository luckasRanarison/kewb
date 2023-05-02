use cube::state::{Corner, Edge};

pub fn co_to_index(corner: &[u8; 8]) -> u16 {
    let mut index: u16 = 0;

    for co in &corner[0..7] {
        index = index * 3 + *co as u16;
    }

    index
}

pub fn index_to_co(mut index: u16) -> [u8; 8] {
    let mut co = [0; 8];
    let mut co_sum = 0;

    for i in (0..7).rev() {
        co[i] = (index % 3) as u8;
        index /= 3;
        co_sum += co[i];
    }
    co[7] = (3 - co_sum % 3) % 3;

    co
}

pub fn eo_to_index(edge: &[u8; 12]) -> u16 {
    let mut index: u16 = 0;

    for eo in &edge[0..11] {
        index = index * 2 + *eo as u16;
    }

    index
}

pub fn index_to_eo(mut index: u16) -> [u8; 12] {
    let mut eo = [0; 12];
    let mut eo_sum = 0;

    for i in (0..11).rev() {
        eo[i] = (index % 2) as u8;
        index /= 2;
        eo_sum += eo[i];
    }
    eo[11] = (2 - eo_sum % 2) % 2;

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

pub fn e_combo_to_index(edge: &[u8; 12]) -> u16 {
    let mut index = 0;
    let mut k = 4;

    for i in (0..12).rev() {
        if edge[i] == 1 {
            index += calculate_combo(i as u8, k);
            k -= 1;
        }
    }

    index
}

pub fn index_to_e_combo(mut index: u16) -> [u8; 12] {
    let mut combo: [u8; 12] = [0; 12];
    let mut k = 4;

    for i in (0..12).rev() {
        if index >= calculate_combo(i, k) {
            combo[i as usize] = 1;
            index -= calculate_combo(i, k);
            k -= 1;
        }
    }

    combo
}

pub fn cp_to_index(cp: &[Corner; 8]) -> u16 {
    let mut index: u16 = 0;

    for i in 0..8 {
        index *= 8 - i as u16;
        for j in i + 1..8 {
            if cp[i] > cp[j] {
                index += 1;
            }
        }
    }

    index
}

pub fn index_to_cp(mut index: u16) -> [Corner; 8] {
    let mut cp: [u8; 8] = [0; 8];

    for i in (0..7).rev() {
        cp[i] = (index % (8 - i as u16)) as u8;
        index /= 8 - i as u16;
        for j in (i as usize + 1)..8 {
            if cp[j] >= cp[i] {
                cp[j] += 1;
            }
        }
    }

    cp.map(|value| Corner::from(value))
}

pub fn ud_ep_to_index(ep: &[Edge; 12]) -> u16 {
    let mut index: u16 = 0;
    let slice = &ep[4..12];

    for i in 0..8 {
        index *= 8 - i as u16;
        for j in i + 1..8 {
            if slice[i] > slice[j] {
                index += 1;
            }
        }
    }

    index
}

pub fn index_to_ud_ep(mut index: u16) -> [Edge; 12] {
    let mut ep = [0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4];
    let slice = &mut ep[4..12];

    for i in (0..7).rev() {
        slice[i] = (index % (8 - i as u16) + 4) as u8;
        index /= 8 - i as u16;
        for j in (i + 1)..8 {
            if slice[j] >= slice[i] {
                slice[j] += 1;
            }
        }
    }

    ep.map(|value| Edge::from(value))
}

pub fn e_ep_to_index(ep: &[Edge; 12]) -> u16 {
    let slice = &ep[0..4];
    let mut index: u16 = 0;

    for i in 0..4 {
        index *= (4 - i as u16) as u16;
        for j in i + 1..4 {
            if slice[i] > slice[j] {
                index += 1;
            }
        }
    }

    index
}

pub fn index_to_e_ep(mut index: u16) -> [Edge; 12] {
    let mut ep = [0, 0, 0, 0, 4, 5, 6, 7, 8, 9, 10, 11];
    let slice = &mut ep[0..4];

    for i in (0..3).rev() {
        slice[i] = (index % (4 - i as u16)) as u8;
        index /= 4 - i as u16;
        for j in (i + 1)..4 {
            if slice[j] >= slice[i] {
                slice[j] += 1;
            }
        }
    }

    ep.map(|value| Edge::from(value))
}

#[cfg(test)]
mod test {
    use super::{co_to_index, index_to_co};
    use crate::index::{
        cp_to_index, e_combo_to_index, e_ep_to_index, eo_to_index, index_to_cp, index_to_e_combo,
        index_to_e_ep, index_to_eo, index_to_ud_ep, ud_ep_to_index,
    };
    use cube::state::{Edge::*, SOLVED_STATE};

    #[test]
    fn test_co_to_index() {
        assert_eq!(co_to_index(&SOLVED_STATE.co), 0);
        assert_eq!(index_to_co(0), SOLVED_STATE.co);

        let co = [2, 0, 0, 1, 1, 0, 0, 2];
        assert_eq!(co_to_index(&co), 1494);
        assert_eq!(index_to_co(1494), co);
    }

    #[test]
    fn test_eo() {
        assert_eq!(eo_to_index(&SOLVED_STATE.eo), 0);
        assert_eq!(index_to_eo(0), SOLVED_STATE.eo);

        let eo = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(eo_to_index(&eo), 2047);
        assert_eq!(index_to_eo(2047), eo);
    }

    #[test]
    fn test_e_combo() {
        let combo = [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(e_combo_to_index(&combo), 0);
        assert_eq!(index_to_e_combo(0), combo);

        let combo = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];
        assert_eq!(e_combo_to_index(&combo), 494);
        assert_eq!(index_to_e_combo(494), combo);
    }

    #[test]
    fn test_cp() {
        assert_eq!(cp_to_index(&SOLVED_STATE.cp), 0);
        assert_eq!(index_to_cp(0), SOLVED_STATE.cp);

        let mut corners = SOLVED_STATE.cp;
        corners.reverse();
        assert_eq!(cp_to_index(&corners), 40319);
        assert_eq!(index_to_cp(40319), corners);
    }

    #[test]
    fn test_ep() {
        assert_eq!(ud_ep_to_index(&SOLVED_STATE.ep), 0);
        assert_eq!(index_to_ud_ep(0), SOLVED_STATE.ep);

        let edges = [BL, BR, FR, FL, DL, DB, DR, DF, UL, UF, UR, UB];
        assert_eq!(ud_ep_to_index(&edges), 40319);
        assert_eq!(index_to_ud_ep(40319), edges);
    }

    #[test]
    fn test_e_ep() {
        assert_eq!(e_ep_to_index(&SOLVED_STATE.ep), 0);
        assert_eq!(index_to_e_ep(0), SOLVED_STATE.ep);

        let edges = [FL, FR, BR, BL, UB, UR, UF, UL, DF, DR, DB, DL];
        assert_eq!(e_ep_to_index(&edges), 23);
        assert_eq!(index_to_e_ep(23), edges);
    }
}
