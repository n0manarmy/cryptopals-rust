use std::cmp::Ordering;
extern crate challenges;
use crate::challenges::utils::{utils, translators};

pub fn run() {
    type Result = (usize, u32, u32, Vec<u8>, Vec<u8>);
    let mut results: Vec<Result> = Vec::new();

    let file_name = "challenge_files/6.txt";
    let enc_str = utils::read_file_by_lines_to_str(&file_name);
    // let enc_str: Vec<u8> = enc_str.chars().map(|x| translators::from_base64(x)).collect();
    dbg!(&enc_str);
    
    const MAX_KEY_SIZE: usize = 40;
    let mut enc_str_pos = 0;

    while enc_str_pos < enc_str.len() {
        // println!("enc_str_pos {} out of {}", enc_str_pos, enc_str.len());

        let mut key_size = 2;
        while key_size <= MAX_KEY_SIZE {
            if key_size < enc_str.len() - enc_str_pos {
                let first: Vec<u8> = utils::collect_bytes(key_size, enc_str_pos, &enc_str.bytes().collect());
                let second: Vec<u8> = utils::collect_bytes(key_size + 1, enc_str_pos, &enc_str.bytes().collect());
                let hammings = utils::hamming_distance(&first, &second);
                // dbg!(hammings);

                results.push((key_size, hammings, hammings / key_size as u32, first, second));
                key_size += 1;
            } else {
                break;
            }
        }

        enc_str_pos += 1;
    }
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    // for r in results {
    //     println!("key_size: {}, hammings: {}, hammings normalize: {}\nfirst: {:?}\nsecond: {:?}", r.0, r.1, r.2, r.3, r.4);
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_ch6_run() {
        run();
    }
}