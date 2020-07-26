use crate::aes::key_expander::expander;
use crate::aes::helper;
use crate::aes::tables;
use crate::aes::printer::print_state;

pub struct Decrypt {
    expanded_key: Vec<u8>,
    rounds: u32,
}

impl Decrypt {

    pub fn new(key: Vec<u8>) -> Decrypt {
        Decrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
        }
    }

    fn get_rounds(key_len: usize) -> u32 {
        match key_len {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("unexpended key size found"),
        }
    }

    pub fn decrypt(self, input: Vec<u8>) -> Vec<u8> {
        print!("start -- input state");
        print_state(&input);

        print!("transform input state");
        let mut state = helper::transform_state(input);
        print_state(&state);

        print!("this round exp key");
        let this_exp_key: Vec<u8> = helper::transform_state(
            helper::get_this_round_exp_key(self.rounds as usize, &self.expanded_key));
        print_state(&this_exp_key);

        print!("start add round key");
        state = helper::add_round_key(state, this_exp_key);
        print_state(&state);

        for x in 1..self.rounds {
            print!("\n{} - round start", x);
            print_state(&state);

            print!("\n{} - inv s_box", x);
            state = state.iter().map(|x| tables::inv_s_box(*x)).collect();
            print_state(&state);

            print!("\n{} - inv shift_rows", x);
            state = helper::inv_shift_rows(state);
            print_state(&state);

            print!("\n{} - inv mix_colums", x);
            state = helper::inv_mix_column(state);
            print_state(&state);
            
            print!("\n{} - this exp key", x);
            let this_exp_key: Vec<u8> = helper::transform_state(
                helper::get_this_round_exp_key((self.rounds - x) as usize, &self.expanded_key));
            print_state(&this_exp_key);

            print!("\n======= {} - add_round =======", x);
            state = helper::add_round_key(state, this_exp_key);
            print_state(&state);
        }

        print!("\n{} - inv_s_box", 0);
        state = state.iter().map(|x| tables::inv_s_box(*x)).collect();
        print_state(&state);

        print!("\n{} - inv shift_rows", 0);
        state = helper::inv_shift_rows(state);
        print_state(&state);

        print!("this exp key");
        let this_exp_key: Vec<u8> = helper::transform_state(
            helper::get_this_round_exp_key(0, &self.expanded_key));
        print_state(&this_exp_key);

        state = helper::add_round_key(state, this_exp_key);        

        // helper::transform_state(output)
        state
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::hex;

    #[test]
    pub fn test_decrypt_128() {
        let cipher_key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        // let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let input: Vec<u8> = vec![0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a];


        let decryptor = Decrypt::new(cipher_key);
        let output: Vec<u8> = decryptor.decrypt(input);

        print_state(&output);
    }

    #[test]
    pub fn test_decrypt_plain_128() {
        println!("build input");
        let input = "69c4e0d86a7b0430d8cdb78070b4c55a";
        let input: Vec<u8> = hex::encoders::str_to_hex_u8_buf(input);
        assert_eq!(input.len(), 16);
        for i in input.clone() {
            print!("{:02x}", i);
        }
        println!("build ciper");
        let cipher = "000102030405060708090a0b0c0d0e0f";
        let cipher: Vec<u8> = hex::encoders::str_to_hex_u8_buf(cipher);
        assert_eq!(cipher.len(), 16);

        let result = "00112233445566778899aabbccddeeff";

        println!("init decryptor");
        let decryptor = Decrypt::new(cipher);
        println!("start decryptor");
        let output: Vec<u8> = decryptor.decrypt(input);
        // let output: String = hex::encoders::hex_buf_to_str(output);
        let output: String = output.iter().map(|x| format!("{:02x}", x)).collect();
        assert_eq!(&output, result);
    }

}