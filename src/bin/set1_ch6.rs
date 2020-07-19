extern crate cryptopals_lib;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::thread;
use crate::cryptopals_lib::base64;
use crate::cryptopals_lib::xor::{helpers, helpers::XorResult};
use crate::cryptopals_lib::utils::{text_utils, encoders, utils, translators};

type KeySizeResult = (usize, u32, f64);
// type XorStore = (usize, usize, f32, BTreeMap<String, f32>, Vec<char>, char);    

pub fn main() {
    let min_key_size = 2;
    let max_key_size = 40;
    let result_scoring_min = 20.0;
    let file_name = "challenge_files/6.txt";
    //read file to string
    let enc_str = utils::read_file_by_lines_to_str(&file_name);
    //base64 decode file
    let enc_str = base64::decoder::decode_str(&enc_str);
    //collect the bytes from the decoded file
    let enc_str: Vec<u8> = enc_str.bytes().collect();


    // //For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, 
    // //and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
    // //The KEYSIZE with the smallest normalized edit distance is probably the key. 
    // //You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances. 
    // let key_size_result: Vec<KeySizeResult> = find_ideal_key_size(min_key_size, max_key_size, &enc_str);
    
    // let optimal_key_count = 4;
    // let mut optimal_key_block_size: Vec<usize> = Vec::new();
    // // collect results of top optimal_key_count optimal key sizes
    // for x in 0..optimal_key_count {
    //     optimal_key_block_size.push(key_size_result[x].0);
    // }

    // let mut xord_buff: Vec<Vec<XorResult>> = Vec::new();
    // //for each keysize, collect the cypher into keysize blocks. then transpose the blocks.
    // //finally for each trnsposed block, solve single char xor
    // for o in optimal_key_block_size {
    //     let collected_cypher = collect_cypher(o, &enc_str);
    //     println!("key block size: {} collected cypher size: {}", o, collected_cypher.len());
        
    //     let transposed = transpose_blocks(o, collected_cypher);
    //     for x in 0..transposed.len() {
    //     // for (i, t) in transposed.iter().enumerate() {
    //         // xord_buff.push(solve_single_char_xor(o, x, &transposed[x]));
    //         let xor: Vec<XorResult> = helpers::single_char_xor(o, x, &transposed[x]);
    //         xord_buff.push(xor.clone());
    //     }
    // }

    // // take transposed results of collected cypher and sort and then print them pretty
    // for mut filtered in xord_buff {
    //     filtered.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal));
    //     let mut count_found = 0;
    //     for f in filtered {
    //         if f.score > result_scoring_min {
    //             // let pretty_str: String = f.2.iter().collect();
    //             println!("--------------------------------------------------------------------------------------------------------------");
    //             // println!("score: {:.2} \t cypher: {} \t decoded: {}", f.0, f.3, pretty_str);
    //             println!("key_size: {}, index: {}, score: {:.2} \t cypher: {}", f.key_size, f.index, f.score, f.key);
    //             println!("--------------------------------------------------------------------------------------------------------------");
    //             count_found += 1;
    //         }
    //     }
    //     if count_found > 0 {
    //         println!("Results found: {}", count_found);
    //     }
    // }

    let key = "in";
    let answer = solve_repeating_xor(key.chars().collect(), &enc_str);
    let decrypt: String = answer.iter().map(|x| *x as char).collect();

    println!("Answer: {}", decrypt);

}

pub fn solve_repeating_xor(key: Vec<char>, cypher: &Vec<u8>) -> Vec<u8>{
    let mut key_pos = 0;
    let mut xord_results: Vec<u8> = Vec::new();
    for c in cypher {
        if key_pos == key.len() {
            key_pos = 0;
        }
        xord_results.push(c ^ key[key_pos] as u8);
    }

    xord_results
}

pub fn filter_results(results: Vec<XorResult>) -> Vec<XorResult> {
    let mut filtered: Vec<XorResult> = Vec::new();
    const MIN_SPACE_COUNT: u32 = 3;

    let filter = false;

    for r in results.iter() {
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

    filtered
}

// Now transpose the blocks: make a block that is the first byte of every block, 
// and a block that is the second byte of every block, and so on. 
fn transpose_blocks(key_block_size: usize, collected_cypher: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = Vec::new();

    for _x in 0..key_block_size {
        transposed.push(Vec::new());
    }

    for cypher in collected_cypher {
        for x in 0..cypher.len() {
            transposed[x].push(cypher[x]);
        }
    }

    transposed
}

fn collect_cypher(key_block_size: usize, enc_str: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut collected: Vec<Vec<u8>> = Vec::new();
    let mut pos = 0;

    //while the decoded bytes are less than the position
    while pos < enc_str.len() {
        let mut key_block_pos = 0;
        let mut vec_buf: Vec<u8> = Vec::new();
        //while the key block pos is less than the key block size we push a block to 
        //the buffer. We then collect the buffer into the vec of bufs
        while key_block_pos < key_block_size && (key_block_pos + pos) < enc_str.len() {
            vec_buf.push(enc_str[key_block_pos + pos]);
            key_block_pos += 1;
        }
        pos += key_block_size;
        collected.push(vec_buf);
    }

    collected
}

// For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, 
// and find the edit distance between them. Normalize this result by dividing by KEYSIZE. 
fn find_ideal_key_size(mut min_key_size: usize, max_key_size: usize, enc_str: &Vec<u8>) -> Vec<KeySizeResult>{
    let mut results: Vec<KeySizeResult> = Vec::new();

    while min_key_size <= max_key_size {
        let enc_str_pos = 0;
        let mut hammings_total = 0;
        //collect first byte chunk of keysize
        let first: Vec<u8> = utils::collect_bytes(min_key_size, enc_str_pos, &enc_str);
        //collect second byte chunk of keysize
        let second: Vec<u8> = utils::collect_bytes(min_key_size, min_key_size, &enc_str);
        //sum the hammings distance between them and devide by key_size
        hammings_total += utils::hamming_distance(&first, &second);
        // save the keysize value, hammings total value, and the the average hammings
        results.push((min_key_size, hammings_total, hammings_total as f64 / min_key_size as f64));
        // println!("computed key_size: {}", min_key_size);
        min_key_size += 1;
    }

    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal));
    let mut results_len = 0;
    for r in &results {
        results_len += 1;
        println!("key_size: {}, hammings: {}, hammings normalize: {:.4}", r.0, r.1, r.2);
    }

    println!("List results len: {}", results_len);
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_ch6_run() {
        main();
    }

    #[test]
    pub fn test_find_ideal_key_size() {
        let file_name = "challenge_files/6.txt";
        //read file to string
        let enc_str = utils::read_file_by_lines_to_str(&file_name);
        //base64 decode file
        let enc_str = base64::decoder::decode_str(&enc_str);
        //collect the bytes from the decoded file
        let enc_str: Vec<u8> = enc_str.bytes().collect();

        //For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, 
        //and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
        //The KEYSIZE with the smallest normalized edit distance is probably the key. 
        //You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances. 
        let min_key_size = 2;
        let max_key_size = 40;
        let key_size_result: Vec<KeySizeResult> = find_ideal_key_size(min_key_size, max_key_size, &enc_str);
    }

    #[test]
    pub fn test_collect_cypher() {
        let test_vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let collected = collect_cypher(3, &test_vec); 
        dbg!(&collected);
        let transposed = transpose_blocks(3, collected);
        dbg!(&transposed);
        for (i, t) in transposed.iter().enumerate() {
            helpers::single_char_xor(3, i, &t);
        }
    }
}