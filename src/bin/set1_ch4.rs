extern crate cryptopals_lib;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::char;
use crate::cryptopals_lib::xor::{helpers, helpers::XorResult};
use crate::cryptopals_lib::utils::{text_utils, utils, encoders};

pub fn main() {
    println!("

    Set 1 Challenge 4
    
    Detect single-character XOR

    One of the 60-character strings in this file has been encrypted by single-character XOR.

    Find it.

    (Your code from #3 should help.)
    
    ");

    let test_file = "challenge_files/4.txt";
    let mut xord_buff: Vec<XorResult> = Vec::new();

    let enc_str = utils::read_file_by_lines_to_vec(test_file);

    for (i, line) in enc_str.iter().enumerate() {
        let hex_enc_str: Vec<u32> = encoders::str_to_hex_val(line.to_string());
        let hex_enc_str: Vec<u8> = hex_enc_str.iter().map(|x| *x as u8).collect();

        let mut temp = helpers::single_char_xor(1, i, &hex_enc_str);
        xord_buff.append(&mut temp);


        // let char_range: Vec<u8> = (0..255).collect();

        // for a in char_range {
        //     count += 1;
        //     let xord_buf: Vec<u32> = hex_enc_str.iter().map(|x| x ^ a as u32).collect();
        //     let decoded: Vec<char> = xord_buf.iter().map(|x| *x as u8 as char).collect();
        //     let this_result = text_utils::ascii_scoring(&decoded);

        //     results.push((i, this_result.0, this_result.1, decoded, a as char));
        // }
    }

    let mut filtered: Vec<XorResult> = Vec::new();
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