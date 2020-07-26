use crate::aes::tables as tbl;

pub fn get_key_sch(round: usize, exp_key: &Vec<u8>) -> Vec<u8> {
    let e_pos = round * 16;
    let mut this_exp_key: Vec<u8> = vec![0;16];
    this_exp_key.clone_from_slice(&exp_key[e_pos..e_pos + 16]);

    // let this_exp_key = transform_state(this_exp_key);

    this_exp_key
}

pub fn add_round_key(mut state: Vec<u8>, exp_key: Vec<u8>) -> Vec<u8>{
    let iter = state.iter().zip(exp_key.iter());
    state = iter.map(|(s, e)| s ^ e).collect::<Vec<u8>>();
    // let mut x = 0;
    // let mut y = 0;
    // for _z in 0..state.len() {
    //     state[xy_idx(x: i32, y: i32)]
    // }

    state
}

pub fn sub_bytes(state: Vec<u8>) -> Vec<u8> {
    let mut t_state: Vec<u8> = vec![0; state.len()];
    for s in state {
        t_state.push(tbl::s_box(s));
    }

    t_state
}

pub fn byte_sub(word: u32) -> u32 {
    let mut temp_vec: Vec<u8> = Vec::new();
    let mut temp_val: u32 = 0;
    temp_vec.push(((word & 0xFF000000) >> 24) as u8);
    temp_vec.push(((word & 0xFF0000) >> 16) as u8);
    temp_vec.push(((word & 0xFF00) >> 8) as u8);
    temp_vec.push((word & 0xFF) as u8);
    for t in 0..temp_vec.len() {
        temp_val |= tbl::s_box(temp_vec[t]) as u32;
        if t == 3 {
            break;
        }
        temp_val <<= 8;
    }

    temp_val

}

pub fn shift_rows(mut state: Vec<u8>) -> Vec<u8> {

    let mut t_state: Vec<u8> = vec![0; 16];
    state = transform_state(state);
    t_state[0] = state[0];
    t_state[1] = state[1];
    t_state[2] = state[2];
    t_state[3] = state[3];

    t_state[4] = state[5];
    t_state[5] = state[6];
    t_state[6] = state[7];
    t_state[7] = state[4];
    
    t_state[8] = state[10];
    t_state[9] = state[11];
    t_state[10] = state[8];
    t_state[11] = state[9];
    
    t_state[12] = state[15];
    t_state[13] = state[12];
    t_state[14] = state[13];
    t_state[15] = state[14];

    // t_state[0] = state[0];
    // t_state[1] = state[4];
    // t_state[2] = state[8];
    // t_state[3] = state[12];

    // t_state[4] = state[5];
    // t_state[5] = state[9];
    // t_state[6] = state[14];
    // t_state[7] = state[1];
    
    // t_state[8] = state[10];
    // t_state[9] = state[14];
    // t_state[10] = state[2];
    // t_state[11] = state[6];
    
    // t_state[12] = state[15];
    // t_state[13] = state[3];
    // t_state[14] = state[7];
    // t_state[15] = state[11];

    // t_state[0] = state[0];
    // t_state[1] = state[5];
    // t_state[2] = state[10];
    // t_state[3] = state[15];

    // t_state[4] = state[5];
    // t_state[5] = state[9];
    // t_state[6] = state[14];
    // t_state[7] = state[3];
    
    // t_state[8] = state[10];
    // t_state[9] = state[13];
    // t_state[10] = state[2];
    // t_state[11] = state[7];
    
    // t_state[12] = state[15];
    // t_state[13] = state[1];
    // t_state[14] = state[6];
    // t_state[15] = state[11];

    transform_state(t_state)
}

pub fn inv_shift_rows(state: Vec<u8>) -> Vec<u8> {

    let mut t_state: Vec<u8> = vec![0; 16];
    // t_state[0] = state[0];
    // t_state[1] = state[1];
    // t_state[2] = state[2];
    // t_state[3] = state[3];

    // t_state[4] = state[7];
    // t_state[5] = state[4];
    // t_state[6] = state[5];
    // t_state[7] = state[6];
    
    // t_state[8] = state[10];
    // t_state[9] = state[11];
    // t_state[10] = state[8];
    // t_state[11] = state[9];
    
    // t_state[12] = state[13];
    // t_state[13] = state[14];
    // t_state[14] = state[15];
    // t_state[15] = state[12];

    t_state[0] = state[0];
    t_state[1] = state[4];
    t_state[2] = state[8];
    t_state[3] = state[12];

    t_state[4] = state[13];
    t_state[5] = state[1];
    t_state[6] = state[5];
    t_state[7] = state[9];
    
    t_state[8] = state[10];
    t_state[9] = state[14];
    t_state[10] = state[2];
    t_state[11] = state[6];
    
    t_state[12] = state[7];
    t_state[13] = state[11];
    t_state[14] = state[15];
    t_state[15] = state[3];

    t_state
}


pub fn xy_idx(x: i32, y: i32) -> usize {
    // println!("x: {} y: {}", x, y);
    (y as usize * 4) + x as usize
}

pub fn transform_state(state: Vec<u8>) -> Vec<u8> {
    let mut t_state: Vec<u8> = vec![0; state.len()];
    t_state[0] = state[0];
    t_state[1] = state[4];
    t_state[2] = state[8];
    t_state[3] = state[12];
    t_state[4] = state[1];
    t_state[5] = state[5];
    t_state[6] = state[9];
    t_state[7] = state[13];
    t_state[8] = state[2];
    t_state[9] = state[6];
    t_state[10] = state[10];
    t_state[11] = state[14];
    t_state[12] = state[3];
    t_state[13] = state[7];
    t_state[14] = state[11];
    t_state[15] = state[15];

    t_state
}

pub fn inv_mix_column(state: Vec<u8>) -> Vec<u8> {
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

pub fn mix_column(state: Vec<u8>) -> Vec<u8> {
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

fn l_box_overflow_check(x: u8, y: u8) -> u8 {
    let mut val: u32 = (x as u32) + (y as u32);
    if val > 0xff {
        // dbg!("Overflow found with {:02x} > 0xff", val);
        val = val - 0xff;
    }
    // println!("sum of {:02x} + {:02x} = {:02x}", x, y, val);
    
    val as u8
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::aes::key_expander;
    use crate::aes::printer::print_state;

    #[test]
    pub fn test_first_add_round() {
        let cipher_key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let expanded: Vec<u8> = key_expander::expander::expand(&cipher_key);
        print_state(&expanded);

        // println!("transform state");
        // let state = transform_state(input);
        // print_state(&state);

        println!("add round key");
        let this_exp_key = get_key_sch(0, &expanded);
        // let this_exp_key = transform_state(this_exp_key);
        let state = add_round_key(input, this_exp_key);
        print_state(&state);
        
    }

    #[test]
    pub fn test_sub_bytes() {
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state = sub_bytes(state);
        print_state(&state);
    }

    #[test]
    pub fn test_mix_column() {
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];
        // let state = transform_state(state);

        let state = mix_column(state);
        print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0x04);
        assert_eq!(state[xy_idx(0, 1)], 0x66);
        assert_eq!(state[xy_idx(0, 2)], 0x81);
        assert_eq!(state[xy_idx(0, 3)], 0xe5);
    }

    #[test]
    pub fn test_inv_mix_column() {
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state: Vec<u8> = vec![0x04, 0x66, 0x81, 0xe5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];

        let state = inv_mix_column(state);
        print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0xd4);
        assert_eq!(state[xy_idx(0, 1)], 0xbf);
        assert_eq!(state[xy_idx(0, 2)], 0x5d);
        assert_eq!(state[xy_idx(0, 3)], 0x30);
    }

    // #[test]
    // pub fn test_transform_state() {
    //     let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    //     let state = transform_state(state);
    //     print_state(&state);
    // }

    #[test]
    pub fn test_inv_s_row() {
        let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let s2: Vec<u8> = vec![0x7a, 0xd5, 0xfd, 0xa7, 0x89, 0xef, 0x4e, 0x27, 0x2b, 0xca, 0x10, 0x0b, 0x3d, 0x9f, 0xf5, 0x9f];
        let s2_r: Vec<u8> = vec![0x7a, 0x9f, 0x10, 0x27, 0x89, 0xd5, 0xf5, 0x0b, 0x2b, 0xef, 0xfd, 0x9f, 0x3d, 0xca, 0x4e, 0xa7];
        let state = inv_shift_rows(state);
        print_state(&state);

        print_state(&s2);
        // let s2 = inv_shift_rows(transform_state(s2));
        // print_state(&s2);
        // let s2 = transform_state(s2);
        // print_state(&s2);
        print_state(&s2_r);
        assert_eq!(s2, s2_r);
    }

    #[test]
    pub fn test_xy_idx() {
        assert_eq!(xy_idx(0, 1), 4)
    }

    #[test]
    pub fn test_shift_row() {
        let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        // let state = transform_state(state);
        let state = shift_rows(state);
        print_state(&state);
    }
}