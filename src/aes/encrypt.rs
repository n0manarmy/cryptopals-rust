use crate::aes::key_expander::expander;
use crate::aes::helper;
use crate::aes::tables;
use crate::aes::printer::print_state;

pub struct Encrypt {
    expanded_key: Vec<u8>,
    rounds: u32,
}

impl Encrypt {

    pub fn new(key: Vec<u8>) -> Encrypt {
        Encrypt {
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

    pub fn encrypt(self, input: Vec<u8>) -> Vec<u8> {
        print!("0 -- input");
        print_state(&input);

        // print!("transform input state");
        // let mut state = helper::transform_state(input);
        // print_state(&state);
        let mut state = input;

        print!("0 - k_sch");
        let ik_sch: Vec<u8> = helper::get_key_sch(0, &self.expanded_key);
        print_state(&ik_sch);
        // let ik_sch = helper::transform_state(ik_sch);

        // print!("start add round key");
        state = helper::add_round_key(state, ik_sch);
        // print_state(&state);

        for x in 1..self.rounds {
            print!("\n{} - start", x);
            print_state(&state);

            print!("\n{} - s_box", x);
            state = state.iter().map(|x| tables::s_box(*x)).collect();
            print_state(&state);

            print!("\n{} - s_row", x);
            state = helper::shift_rows(state);
            print_state(&state);

            print!("\n{} - m_col", x);
            state = helper::mix_column(state);
            print_state(&state);

            print!("\n{} - k_sch", x);
            // let ik_sch: Vec<u8> = helper::transform_state(
                // helper::get_this_round_exp_key(x as usize, &self.expanded_key));
            
            let ik_sch: Vec<u8> = helper::get_key_sch(x as usize, &self.expanded_key);
            print_state(&ik_sch);

            print!("\n{} - k_add", x);
            state = helper::add_round_key(state, ik_sch);
            print_state(&state);
        }


        print!("\n{} - s_box", self.rounds);
        state = state.iter().map(|x| tables::s_box(*x)).collect();
        print_state(&state);

        print!("\n{} - s_row", self.rounds);
        state = helper::shift_rows(state);
        print_state(&state);

        print!("k_sch");
        // let ik_sch: Vec<u8> = helper::transform_state(helper::get_this_round_exp_key(self.rounds as usize, &self.expanded_key));
        let ik_sch: Vec<u8> = helper::get_key_sch(self.rounds as usize, &self.expanded_key);
        print_state(&ik_sch);

        state = helper::add_round_key(state, ik_sch);        

        // helper::transform_state(output)  
        state
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::hex;
    use crate::aes::printer::print_state;

    #[test]
    pub fn test_encrypt_128() {
        let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let cipher_key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let result: Vec<u8> = vec![0x39, 0x02, 0xdc, 0x19, 0x25, 0xdc, 0x11, 0x6a, 0x84, 0x09, 0x85, 0x0b, 0x1d, 0xfb, 0x97, 0x32];

        let encryptor = Encrypt::new(cipher_key);
        let output: Vec<u8> = encryptor.encrypt(input);

        print_state(&output);

        assert_eq!(output, result);
    }

    #[test]
    pub fn test_encrypt_plain_128() {
        let input = "00112233445566778899aabbccddeeff";
        let input: Vec<u8> = hex::encoders::str_to_hex_u8_buf(input);
        let cipher = "000102030405060708090a0b0c0d0e0f";
        let cipher: Vec<u8> = hex::encoders::str_to_hex_u8_buf(cipher);

        let result = "69c4e0d86a7b0430d8cdb78070b4c55a";

        let encryptor = Encrypt::new(cipher);
        // let output: Vec<u8> = helper::transform_state(encryptor.encrypt(input));
        let output: Vec<u8> = encryptor.encrypt(input);
        print_state(&output);
        let output: String = output.iter().map(|x| format!("{:02x}", x)).collect();
        println!("output: {}", &output);
        assert_eq!(&output, result);
    }

}