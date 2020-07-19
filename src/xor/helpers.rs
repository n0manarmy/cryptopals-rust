use std::collections::BTreeMap;
use crate::utils::{text_utils, utils, encoders};

#[derive(Clone)]
pub struct XorResult {
    pub key_size: usize,
    pub index: usize,
    pub score: f32,
    pub b_tree_map_result: BTreeMap<String, f32>,
    pub decode: Vec<char>,
    pub key: char,
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