use crate::challenges::utils::{translators, encoders, text_utils};
use std::collections::BTreeMap;
use std::char;

pub fn run() {
    println!(
        "
        Chapter 1 Challenge 3
        
        Single-byte XOR cipher

        The hex encoded string:

        1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

        ... has been XOR'd against a single character. Find the key, decrypt the message.

        You can do this by hand. But don't: write code to do it for you.

        How? Devise some method for \"scoring\" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
        Achievement Unlocked

        You now have our permission to make \"ETAOIN SHRDLU\" jokes on Twitter.
        ");
    let hex_enc_str = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let hex_enc_str: Vec<u32> = encoders::str_to_hex_val(hex_enc_str);
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    type Result = (f32, BTreeMap<String, f32>, Vec<char>, char);

    let mut results: Vec<Result> = Vec::new();

    for a in alpha.bytes() {
        // xor buffer with char
        let xord_buf: Vec<u32> = hex_enc_str.iter().map(|x| *x as u32 ^ a as u32).collect();

        // convert xor'd buffer to ascii
        let decoded: Vec<char> = xord_buf.iter().map(|x| {
            let bytes_to_hex = format!("{:X}", x);
            translators::hex_string_to_ascii(bytes_to_hex)

        }).collect();

        let this_result = text_utils::ascii_scoring(&decoded);

        results.push((this_result.0, this_result.1, decoded, a as char));
    }

    // results.sort_by();
    const MIN_SPACE_COUNT: u32 = 3;
    for r in results {
        if text_utils::non_standard_single_letter(&r.2) {
            continue;
        }
        if text_utils::count_spaces(&r.2) < MIN_SPACE_COUNT {
            continue;
        }
        if text_utils::garbage_found(&r.2) {
            continue;
        }
        if r.0 > 1.0 {
            let pretty_str: String = r.2.iter().collect();
            println!("score: {:.2} \t cypher: {} \t decoded: {}", r.0, r.3, pretty_str);
        }
    }
}