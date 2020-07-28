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



pub fn l_box_overflow_check(x: u8, y: u8) -> u8 {
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

    #[test]
    pub fn test_xy_idx() {
        assert_eq!(xy_idx(0, 1), 4)
    }


}