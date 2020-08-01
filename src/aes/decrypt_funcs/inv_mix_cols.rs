use crate::aes::tables as tbl;
use crate::aes::helper::{xy_idx, t_xy_idx};
use crate::aes::helper::overflow_check;

pub fn mixV2(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut inv_m_col = 0;
    let inv_m_row = 0;
    let mut s_pos: i32 = 0;
    let x = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {

        t_state[xy_idx(0, y)] = p_operate(&state, y, inv_m_col);
        inv_m_col += 1;
        t_state[xy_idx(1, y)] = p_operate(&state, y, inv_m_col);
        inv_m_col += 1;
        t_state[xy_idx(2, y)] = p_operate(&state, y, inv_m_col);
        inv_m_col += 1;
        t_state[xy_idx(3, y)] = p_operate(&state, y, inv_m_col);
        inv_m_col = 0;
        s_pos += 4;
        y += 1;
    }

    t_state
}


fn p_operate(state: &Vec<u8>, y: i32, inv_m_col: usize) -> u8 {
    let t1 = overflow_check(state[xy_idx(0, y)], tbl::inv_m_mtrx(0, inv_m_col));
    let t2 = overflow_check(state[xy_idx(1, y)], tbl::inv_m_mtrx(1, inv_m_col));
    let t3 = overflow_check(state[xy_idx(2, y)], tbl::inv_m_mtrx(2, inv_m_col));
    let t4 = overflow_check(state[xy_idx(3, y)], tbl::inv_m_mtrx(3, inv_m_col));
    // println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
    t1 ^ t2 ^ t3 ^ t4
}

fn l_operate(state: &Vec<u8>, y: i32, inv_m_col: usize) -> u8 {
    let t1 = overflow_check(tbl::l_box(state[xy_idx(0, y)]), tbl::l_box(tbl::inv_m_mtrx(0, inv_m_col)));
    let t2 = overflow_check(tbl::l_box(state[xy_idx(1, y)]), tbl::l_box(tbl::inv_m_mtrx(1, inv_m_col)));
    let t3 = overflow_check(tbl::l_box(state[xy_idx(2, y)]), tbl::l_box(tbl::inv_m_mtrx(2, inv_m_col)));
    let t4 = overflow_check(tbl::l_box(state[xy_idx(3, y)]), tbl::l_box(tbl::inv_m_mtrx(3, inv_m_col)));
    println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
    println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", tbl::e_box(t1), tbl::e_box(t2), tbl::e_box(t3), tbl::e_box(t4));
    tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4)
}

pub fn mix(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut inv_m_col = 0;
    let inv_m_row = 0;
    let mut s_pos: i32 = 0;
    let x = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {

        let t1 = overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", tbl::e_box(t1), tbl::e_box(t2), tbl::e_box(t3), tbl::e_box(t4));
        t_state[xy_idx(x, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        println!("{:02x}" , t_state[xy_idx(x + 1, y)]);

        inv_m_col += 1;

        let t1 = overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", tbl::e_box(t1), tbl::e_box(t2), tbl::e_box(t3), tbl::e_box(t4));
        t_state[xy_idx(x + 1, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        println!("{:02x}" , t_state[xy_idx(x + 1, y)]);
        
        inv_m_col += 1;

        let t1 = overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", tbl::e_box(t1), tbl::e_box(t2), tbl::e_box(t3), tbl::e_box(t4));
        t_state[xy_idx(x + 2, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        println!("{:02x}" , t_state[xy_idx(x + 2, y)]);
        
        inv_m_col += 1;

        let t1 = overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 0, inv_m_col)));
        let t2 = overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 1, inv_m_col)));
        let t3 = overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 2, inv_m_col)));
        let t4 = overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]), tbl::l_box(tbl::inv_m_mtrx(inv_m_row + 3, inv_m_col)));

        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
        println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", tbl::e_box(t1), tbl::e_box(t2), tbl::e_box(t3), tbl::e_box(t4));
        t_state[xy_idx(x + 3, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        println!("{:02x}" , t_state[xy_idx(x + 3, y)]);
        
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

    #[test]
    pub fn test_round_5_col_mix() {
        let state: Vec<u8> = vec![0x98, 0x16, 0xee, 0x74, 0x00, 0xf8, 0x7f, 0x55, 0x6b, 0x2c, 0x04, 0x9c, 0x8e, 0x5a, 0xd0, 0x36];
        let result: Vec<u8> = vec![0xe8, 0xda, 0xb6, 0x90, 0x14, 0x77, 0xd4, 0x65, 0x3f, 0xf7, 0xf5, 0xe2, 0xe7, 0x47, 0xdd, 0x4f];
        let state = mix(state);
        print_state(&state);
        assert_eq!(state, result);
    }
}