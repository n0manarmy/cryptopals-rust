extern crate challenges;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::char;
use crate::challenges::utils::{text_utils, utils, encoders};

pub fn run() {
    println!("

    Set 1 Challenge 4
    
    Detect single-character XOR

    One of the 60-character strings in this file has been encrypted by single-character XOR.

    Find it.

    (Your code from #3 should help.)
    
    ");
    type Result = (usize, f32, BTreeMap<String, f32>, Vec<char>, char);    

    let test_file = "challenge_files/4.txt";
    let mut results: Vec<Result> = Vec::new();
    let mut count = 0;

    let enc_str = utils::read_file_by_lines_to_vec(test_file);

    for (i, line) in enc_str.iter().enumerate() {
        let hex_enc_str: Vec<u32> = encoders::str_to_hex_val(line.to_string());

        let char_range: Vec<u8> = (0..255).collect();

        for a in char_range {
            count += 1;
            let xord_buf: Vec<u32> = hex_enc_str.iter().map(|x| x ^ a as u32).collect();
            let decoded: Vec<char> = xord_buf.iter().map(|x| *x as u8 as char).collect();
            let this_result = text_utils::ascii_scoring(&decoded);

            results.push((i, this_result.0, this_result.1, decoded, a as char));
        }
    }

    println!("Total iterations {}", count);

    let mut filtered: Vec<Result> = Vec::new();
    const MIN_SPACE_COUNT: u32 = 3;

    let filter = true;

    for r in results.iter() {
        if filter {
            if text_utils::non_standard_single_letter(&r.3) {
                continue;
            }
            if text_utils::count_spaces(&r.3) < MIN_SPACE_COUNT {
                continue;
            }
            if text_utils::garbage_found(&r.3) {
                continue;
            }
        }
        filtered.push(r.clone());
    }

    filtered.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    let mut count_found = 0;
    for f in filtered {
        if f.1 > 1.0 {
            let pretty_str: String = f.3.iter().collect();
            println!("index: {} \t score: {:.2} \t cypher: {} \t decoded: {}", f.0, f.1, f.4, pretty_str );
            count_found += 1;
        }
    }

    println!("Results found: {}", count_found);
}