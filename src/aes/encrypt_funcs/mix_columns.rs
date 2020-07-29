use crate::aes::helper::{l_box_overflow_check, xy_idx, t_xy_idx};
use crate::aes::tables as tbl;

pub fn mix(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut m_col = 0;
    let m_row = 0;
    let mut s_pos: i32 = 0;
    let x = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {
        if m_col == 3 {
            m_col = 0;
        }
        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 0, y)]), tbl::l_box(tbl::m_mtrx(m_row, m_col)));
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 1, m_col)));
        let t3 = state[xy_idx(x + 2, y)];
        let t4 = state[xy_idx(x + 3, y)];
        t_state[xy_idx(x, y)] = tbl::e_box(t1) ^ tbl::e_box(t2) ^ t3 ^ t4;
        // println!("1 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y)], x, y);
        
        m_col += 1;
        let t1 = state[xy_idx(x + 0, y)];
        let t2 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 1, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 1, m_col)));
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 2, m_col)));
        let t4 = state[xy_idx(x + 3, y)];
        t_state[xy_idx(x + 1, y)] = t1 ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ t4;
        // println!("2 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 1)], x, y);

        m_col += 1;
        let t1 = state[xy_idx(x + 0, y)];
        let t2 = state[xy_idx(x + 1, y)];
        let t3 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 2, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 2, m_col)));
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 3, m_col)));
        t_state[xy_idx(x + 2, y)] = t1 ^ t2 ^ tbl::e_box(t3) ^ tbl::e_box(t4);
        // println!("3 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 2)], x, y);
        
        m_col += 1;
        let t1 = l_box_overflow_check(tbl::l_box(state[xy_idx(x, y)])     , tbl::l_box(tbl::m_mtrx(m_row, m_col)));
        let t2 = state[xy_idx(x + 1, y)];
        let t3 = state[xy_idx(x + 2, y)];
        let t4 = l_box_overflow_check(tbl::l_box(state[xy_idx(x + 3, y)]) , tbl::l_box(tbl::m_mtrx(m_row + 3, m_col)));
        t_state[xy_idx(x + 3, y)] = tbl::e_box(t1) ^ t2 ^ t3 ^ tbl::e_box(t4);
        
        // println!("4 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 3)], x, y);
        
        s_pos += 4;
        y += 1;
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
    pub fn test_mix_column() {
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];
        // let state = transform_state(state);

        let state = mix(state);
        print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0x04);
        assert_eq!(state[xy_idx(0, 1)], 0x66);
        assert_eq!(state[xy_idx(0, 2)], 0x81);
        assert_eq!(state[xy_idx(0, 3)], 0xe5);
    }
}