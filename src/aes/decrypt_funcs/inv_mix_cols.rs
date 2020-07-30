use crate::aes::tables as tbl;
use crate::aes::helper::{xy_idx, t_xy_idx};
use crate::aes::helper::l_box_overflow_check;

pub fn mix(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut inv_m_col = 0;
    let inv_m_row = 0;
    let mut s_pos: i32 = 0;
    let x = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {
        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        t_state[xy_idx(x, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);

        inv_m_col += 1;

        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        t_state[xy_idx(x + 1, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        
        inv_m_col += 1;

        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        t_state[xy_idx(x + 2, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        
        inv_m_col += 1;

        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        t_state[xy_idx(x + 3, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        
        inv_m_col = 0;
        
        s_pos += 4;
        y += 1;
    }

    t_state
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::aes::printer::print_state;


    #[test]
    pub fn test_inv_mix_column() {
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state: Vec<u8> = vec![0x04, 0x66, 0x81, 0xe5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];

        let state = mix(state);
        print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0xd4);
        assert_eq!(state[xy_idx(0, 1)], 0xbf);
        assert_eq!(state[xy_idx(0, 2)], 0x5d);
        assert_eq!(state[xy_idx(0, 3)], 0x30);
    }
}