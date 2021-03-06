use std::collections::BTreeMap;
use crate::utils::text_utils;

#[derive(Clone)]
pub struct XorResult {
    pub key_size: usize,
    pub index: usize,
    pub score: f32,
    pub b_tree_map_result: BTreeMap<String, f32>,
    pub decode: Vec<char>,
    pub key: char,
}

pub fn xor_same_size_buffers(buf1: Vec<u32>, buf2: Vec<u32>) -> Vec<u32> {
    let mut xor_results: Vec<u32> = Vec::new();
    let mut pos = 0;
    if buf1.len() != buf2.len() {
        panic!("buffers are not equal length");
    };

    while pos < buf1.len() {
        xor_results.push(buf1[pos] ^ buf2[pos]);
        pos += 1;
    }

    xor_results

}

pub fn xor_buf_against_value(buf: &Vec<u32>, val: u32) -> Vec<u32> {
    let mut xor_results: Vec<u32> = Vec::new();
    for b in buf {
        xor_results.push(b ^ val)
    }

    xor_results
}

pub fn single_char_xor(key_size: usize, index: usize, cypher: &Vec<u8>) -> Vec<XorResult> {
    let char_range: Vec<u8> = (0..255).collect();
    let mut xord_results: Vec<XorResult> = Vec::new();

    for a in char_range {
        let xord_buf: Vec<char> = cypher.iter().map(|x| (x ^ a) as char).collect();
        let this_score = text_utils::ascii_scoring(&xord_buf);
        xord_results.push(XorResult {
            key_size,
            index, 
            score: this_score.0, 
            b_tree_map_result: this_score.1, 
            decode: xord_buf, 
            key: a as char,
        });
    }

    xord_results
}

pub fn solve_repeating_xor(key: &Vec<char>, cypher: &Vec<u8>) -> Vec<u8>{
    let mut key_pos = 0;
    let mut xord_results: Vec<u8> = Vec::new();
    for c in cypher {
        if key_pos == key.len() {
            key_pos = 0;
        }
        xord_results.push(c ^ key[key_pos] as u8);
        key_pos += 1;
    }

    xord_results
}