extern crate cryptopals_lib;
use crate::cryptopals_lib::base64;
use crate::cryptopals_lib::utils::file_io_utils;

pub fn main() {
    let file_name = "challenge_files/7.txt";

    //read file to string
    let enc_str = file_io_utils::read_file_by_lines_to_str(&file_name);
    //base64 decode file
    let enc_str = base64::decoder::decode_str(&enc_str);
    //collect the bytes from the decoded file
    let enc_str: Vec<u8> = enc_str.bytes().collect();
}