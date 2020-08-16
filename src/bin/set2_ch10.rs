/*
Implement CBC mode

CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, 
despite the fact that a block cipher natively only transforms individual blocks.

In CBC mode, each ciphertext block is added to the next plaintext block before the 
next call to the cipher core.

The first plaintext block, which has no associated previous ciphertext block, is 
added to a "fake 0th ciphertext block" called the initialization vector, or IV.

Implement CBC mode by hand by taking the ECB function you wrote earlier, 
making it encrypt instead of decrypt (verify this by decrypting whatever you encrypt to test), 
and using your XOR function from the previous exercise to combine them.

The file here is intelligible (somewhat) when CBC decrypted against "YELLOW SUBMARINE" 
with an IV of all ASCII 0 (\x00\x00\x00 &c) Don't cheat.

Do not use OpenSSL's CBC code to do CBC mode, even to verify your results. 
What's the point of even doing this stuff if you aren't going to learn from it?
*/

extern crate rusty_aes;
extern crate cryptopals_lib;
use crate::rusty_aes::{aes_mode::AesMode, decrypt::Decrypt};
use crate::cryptopals_lib::utils::file_io_utils;
use crate::cryptopals_lib::base64::decoder;

fn main() {
    let key = "YELLOW SUBMARINE".as_bytes().to_vec();
    let file = "challenge_files/10.txt";
    let iv = "0000000000000000".as_bytes().to_vec();
    let input = file_io_utils::read_file_by_lines_to_str(file);
    let input = decoder::decode_str_to_u8(&input);
    let decryptor: Decrypt = Decrypt::new(key, AesMode::CBC);
    let results = decryptor.start_cbc(input, iv);

    for r in results {
        print!("{}", r as char);
    }
}