use crate::aes::tables as tbl;
use crate::aes::helper::{xy_idx, t_xy_idx};
use crate::aes::helper::l_box_overflow_check;

pub fn mix(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut inv_m_col = 0;
    let inv_m_row = 0;
    let mut s_pos: i32 = 0;
    let mut x = 0;
    let y = 0;

    while s_pos < state.len() as i32 {
        if inv_m_col == 3 {
            inv_m_col = 0;
        }
        if x == 4 {
            x = 0;
        }
        t_state[xy_idx(x, y + 0)] =
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 0)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col))));
        
        inv_m_col += 1;
        // println!("1 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y)], x, y);
        

        t_state[xy_idx(x, y + 1)] = 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 0)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col))));
        
        inv_m_col += 1;
        // println!("2 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 1)], x, y);

        t_state[xy_idx(x, y + 2)] = 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 0)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col))));
        inv_m_col += 1;
        // println!("3 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 2)], x, y);

        t_state[xy_idx(x, y + 3)] = 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 0)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]) , tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col))));
        
        // println!("4 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 3)], x, y);
        
        s_pos += 4;
        x += 1;
        // dbg!(s_pos);
        // dbg!(x);
        // dbg!(&t_state);
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