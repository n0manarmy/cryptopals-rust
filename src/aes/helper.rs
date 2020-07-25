use crate::aes::tables as tbl;

pub fn add_round_key(state: Vec<u8>, exp_key: Vec<u8>) -> Vec<u8>{
    let iter = state.iter().zip(exp_key.iter());
    
    iter.map(|(s, e)| s ^ e).collect::<Vec<u8>>()
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

pub fn shift_rows(state: Vec<u8>) -> Vec<u8> {

    let mut t_state: Vec<u8> = vec![0; 16];
    t_state[0] = state[0];
    t_state[1] = state[4];
    t_state[2] = state[8];
    t_state[3] = state[12];
    t_state[4] = state[5];
    t_state[5] = state[9];
    t_state[6] = state[13];
    t_state[7] = state[1];
    t_state[8] = state[10];
    t_state[9] = state[14];
    t_state[10] = state[2];
    t_state[11] = state[6];
    t_state[12] = state[15];
    t_state[13] = state[3];
    t_state[14] = state[7];
    t_state[15] = state[11];

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

pub fn mix_column(state: Vec<u8>) -> Vec<u8> {
    println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut m_pos = (0, 0);
    let mut s_pos: i32 = 0;
    let mut x = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {
        if m_pos.1 == 3 {
            m_pos.1 = 0;
        }
        t_state[xy_idx(x, y)] =
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y)]), tbl::l_box(tbl::m_mtrx(m_pos.0, m_pos.1)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 1, m_pos.1)))) ^ 
            state[xy_idx(x, y + 2)] ^
            state[xy_idx(x, y + 3)];
        
        m_pos.1 += 1;
        println!("1 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y)], x, y);
        

        t_state[xy_idx(x, y + 1)] = 
            state[xy_idx(x, y)] ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 1, m_pos.1)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 2, m_pos.1)))) ^ 
            state[xy_idx(x, y + 3)];
        
        m_pos.1 += 1;
        println!("2 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 1)], x, y);

        t_state[xy_idx(x, y + 2)] = 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y)])     , tbl::l_box(tbl::m_mtrx(m_pos.0, m_pos.1)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 1, m_pos.1)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 2, m_pos.1)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 3, m_pos.1))));

        m_pos.1 += 1;
        println!("3 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 2)], x, y);

        t_state[xy_idx(x, y + 3)] = 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y)])     , tbl::l_box(tbl::m_mtrx(m_pos.0, m_pos.1)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 1)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 1, m_pos.1)))) ^
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 2)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 2, m_pos.1)))) ^ 
            tbl::e_box(
                l_box_overflow_check(
                    tbl::l_box(state[xy_idx(x, y + 3)]) , tbl::l_box(tbl::m_mtrx(m_pos.0 + 3, m_pos.1))));
        
        println!("4 -- t_state: {:02x} at {},{}", t_state[xy_idx(x, y + 3)], x, y);
        
        s_pos += 4;
        x += 1;
        // dbg!(s_pos);
        // dbg!(x);
        // dbg!(&t_state);
    }

    t_state
}

fn l_box_overflow_check(x: u8, y: u8) -> u8 {
    let mut val: u32 = (x as u32) + (y as u32);
    if val > 0xff {
        dbg!("Overflow found with {:02x} > 0xff", val);
        val = val - 0xff;
    }
    println!("sum of {:02x} + {:02x} = {:02x}", x, y, val);
    
    val as u8
}

// pub fn mix_column(state: Vec<u8>) -> Vec<u8> {
//     // t_state[0] = (
//     //     t_state[xy_idx(0, 0)] * [t_state[xy_idx(0, 0)]]) 

//     let mut t_state: Vec<u8> = vec![0;state.len()];
//     // matrix pos tracker
//     let mut m_pos = (0, 0);
//     let mut s_pos: i32 = 0;
//     let mut x = 0;
//     let mut y = 0;

//     while s_pos < state.len() as i32 {
//         if m_pos.0 == 3 {
//             m_pos.0 = 0;
//         }
//         // dbg!(&t_state, &state, x, y, s_pos, m_pos.0, m_pos.1);

//         // dbg!(state[xy_idx(x, y)]);

//         t_state[xy_idx(x, y)] =  
//             (state[xy_idx(x, y)]      * tbl::m_mtrx(m_pos.0, m_pos.1)) ^ 
//             (state[xy_idx(x, y + 1)]  * tbl::m_mtrx(m_pos.0, m_pos.1 + 1)) ^
//             (state[xy_idx(x, y + 2)]  * tbl::m_mtrx(m_pos.0, m_pos.1 + 2)) ^ 
//             (state[xy_idx(x, y + 3)]  * tbl::m_mtrx(m_pos.0, m_pos.1 + 3));
//         m_pos.0 += 1;

//         dbg!(&t_state);

//         t_state[xy_idx(x, y + 1)] = 
//             (state[xy_idx(x, y)]     * tbl::m_mtrx(m_pos.0, m_pos.1)) ^ 
//             (state[xy_idx(x, y + 1)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 1)) ^
//             (state[xy_idx(x, y + 2)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 2)) ^ 
//             (state[xy_idx(x, y + 3)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 3));
//         m_pos.0 += 1;

//         dbg!(&t_state);


//         t_state[xy_idx(x, y + 2)] = 
//             (state[xy_idx(x, y)]     * tbl::m_mtrx(m_pos.0, m_pos.1)) ^ 
//             (state[xy_idx(x, y + 1)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 1)) ^
//             (state[xy_idx(x, y + 2)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 2)) ^ 
//             (state[xy_idx(x, y + 3)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 3));
//         m_pos.0 += 1;
        
//         dbg!(&t_state);

//         t_state[xy_idx(x, y + 3)] = 
//             (state[xy_idx(x, y)]     * tbl::m_mtrx(m_pos.0, m_pos.1)) ^ 
//             (state[xy_idx(x, y + 1)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 1)) ^
//             (state[xy_idx(x, y + 2)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 2)) ^ 
//             (state[xy_idx(x, y + 3)] * tbl::m_mtrx(m_pos.0, m_pos.1 + 3));
        
//         dbg!(&t_state);
        
//         s_pos += 4;
//         x += 1;
//         dbg!(s_pos);
//         dbg!(x);
//         // dbg!(&t_state);
//     }

//     t_state
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_mix_column() {
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];
        let state = transform_state(state);
        assert_eq!(state[xy_idx(0, 0)], 0xd4);
        assert_eq!(state[xy_idx(0, 1)], 0xbf);
        assert_eq!(state[xy_idx(0, 2)], 0x5d);
        assert_eq!(state[xy_idx(0, 3)], 0x30);

        let state = mix_column(state);
        for x in 0..state.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02x} ", state[x]);
        }
        println!();

        assert_eq!(state[xy_idx(0, 0)], 0x04);
        assert_eq!(state[xy_idx(0, 1)], 0x66);
        assert_eq!(state[xy_idx(0, 2)], 0x81);
        assert_eq!(state[xy_idx(0, 3)], 0xe5);
    }

    #[test]
    pub fn test_transform_state() {
        let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let state = transform_state(state);
        for x in 0..state.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02} ", state[x]);
        }
        println!();
    }

    #[test]
    pub fn test_xy_idx() {
        assert_eq!(xy_idx(0, 1), 4)
    }

    #[test]
    pub fn test_shift_row() {
        let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let state = shift_rows(state);
        for x in 0..state.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02} ", state[x]);
        }
        println!();
    }

    #[test]
    pub fn test_add_round_key() {
        let state: Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x0a];
        let exp_key: Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x0a];
        let result: Vec<u8> = vec![0x0, 0x0, 0x0, 0x0, 0x0];
        assert_eq!(add_round_key(state, exp_key), result);
        
        let state: Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x0a];
        let exp_key: Vec<u8> = vec![0x09, 0x19, 0x29, 0x39, 0x09];
        let result: Vec<u8> = vec![0x19, 0x39, 0x19, 0x79, 0x03];
        assert_eq!(add_round_key(state, exp_key), result);

    }
}