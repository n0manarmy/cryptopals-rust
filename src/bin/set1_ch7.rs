extern crate cryptopals_lib;
extern crate rusty_aes;
use crate::rusty_aes::aes_mode::AesMode;
use crate::cryptopals_lib::base64;
use crate::cryptopals_lib::utils::file_io_utils;
use crate::rusty_aes::decrypt::Decrypt;
use crate::cryptopals_lib::hex;

pub fn main() {
    let file_name = "challenge_files/7.txt";
    
    //read file to string
    let input = file_io_utils::read_file_by_lines_to_str(&file_name);

    //base64 decode file
    let input = base64::decoder::decode_str_to_u8(&input);

    let key = "YELLOW SUBMARINE".as_bytes().to_vec();
    
    //instantiate our aes decryptor
    let decrypt: Decrypt = Decrypt::new(key, AesMode::ECB);
    let results = decrypt.start_ecb(input);
    
    // let mut buf: Vec<u8> = Vec::new();
    // let buf_len = 16;
    // println!();
    // let mut count = 0;
    // while count < input.len() {
    //     if count + buf_len >= input.len() {
    //         // buf = enc_str[count..(enc_str.len() - count)].to_vec();
    //         let mut slice = input[count..count + (input.len() - count)].to_vec();
    //         let padding = buf_len - slice.len() ;
    //         for _z in 0..padding {
    //             slice.push(0x80);
    //         }
    //         buf.append(&mut decrypt.decrypt(slice));
    //     } 
    //     else {
    //         let slice = input[count..(count + buf_len)].to_vec();
    //         assert_eq!(slice.len(), buf_len);
    //         buf.append(&mut decrypt.decrypt(slice));
    //     }
    //     count += buf_len;
    // }
    for r in results {
        print!("{}", r as char);
    }
    println!();
}