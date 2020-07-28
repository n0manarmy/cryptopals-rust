
pub fn add(mut state: Vec<u8>, exp_key: Vec<u8>) -> Vec<u8>{
    let iter = state.iter().zip(exp_key.iter());
    state = iter.map(|(s, e)| s ^ e).collect::<Vec<u8>>();
    // let mut x = 0;
    // let mut y = 0;
    // for _z in 0..state.len() {
    //     state[xy_idx(x: i32, y: i32)]
    // }

    state
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::aes::printer::print_state;
    use crate::aes::key_expander;
    use crate::aes::encrypt_funcs::key_sch;

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
        let this_exp_key = key_sch::get(0, &expanded);
        // let this_exp_key = transform_state(this_exp_key);
        let state = add(input, this_exp_key);
        print_state(&state);
        
    }

}