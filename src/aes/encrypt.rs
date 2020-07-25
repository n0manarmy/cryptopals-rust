use crate::aes::key_expander::expander;
use crate::aes::helper;

pub struct Encrypt {
    expanded_key: Vec<u8>,
}

impl Encrypt {

    pub fn new(key: Vec<u8>) -> Encrypt {
        Encrypt {
            expanded_key: expander::expand(&key),
        }
    }

    pub fn encrypt(&self, input: Vec<u8>) -> Vec<u8> {
        let mut state: Vec<u8> = helper::transform_state(input);

        state
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_init_ciphers() {
        let val: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let encryptor = Encrypt::new(val);
        let output: Vec<u8> = encryptor.encrypt(input);
    }

}