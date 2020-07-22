extern crate cryptopals_lib;
use std::cmp::Ordering;
use crate::cryptopals_lib::{xor, hex};
use crate::cryptopals_lib::utils::{text_utils, file_io_utils};

pub fn main() {
    println!("

    Set 1 Challenge 4
    
    Detect single-character XOR

    One of the 60-character strings in this file has been encrypted by single-character XOR.

    Find it.

    (Your code from #3 should help.)
    
    ");

    let test_file = "challenge_files/4.txt";
    let mut xord_buff: Vec<xor::helpers::XorResult> = Vec::new();

    //read file into buffer
    let enc_str = file_io_utils::read_file_by_lines_to_vec(test_file);

    for (i, line) in enc_str.iter().enumerate() {
        //decode hex values to bytes
        let hex_enc_str: Vec<u32> = hex::encoders::str_to_hex_val(line.to_string());
        let hex_enc_str: Vec<u8> = hex_enc_str.iter().map(|x| *x as u8).collect();

        let mut temp = xor::helpers::single_char_xor(1, i, &hex_enc_str);
        xord_buff.append(&mut temp);
    }

    let mut filtered: Vec<xor::helpers::XorResult> = Vec::new();
    const MIN_SPACE_COUNT: u32 = 3;

    let filter = true;

    for r in xord_buff.iter() {
        if filter {
            if text_utils::non_standard_single_letter(&r.decode) {
                continue;
            }
            if text_utils::count_spaces(&r.decode) < MIN_SPACE_COUNT {
                continue;
            }
            if text_utils::garbage_found(&r.decode) {
                continue;
            }
        }
        filtered.push(r.clone());
    }

    filtered.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal));
    let mut count_found = 0;
    for f in filtered {
        if f.score > 1.0 {
            let pretty_str: String = f.decode.iter().collect();
            println!("index: {} \t score: {:.2} \t cypher: {} \t decoded: {}", f.index, f.score, f.key, pretty_str );
            count_found += 1;
        }
    }

    println!("Results found: {}", count_found);
}