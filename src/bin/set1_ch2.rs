extern crate cryptopals_lib;
use crate::cryptopals_lib::utils::{translators};
use crate::cryptopals_lib::xor;

pub fn main() {
    println!(
        "
        Chapter 1 Challenge 2
        
        Fixed XOR

        Write a function that takes two equal-length buffers and produces their XOR combination.

        If your function works properly, then when you feed it the string:

        1c0111001f010100061a024b53535009181c

        ... after hex decoding, and when XOR'd against:

        686974207468652062756c6c277320657965

        ... should produce:

        746865206b696420646f6e277420706c6179

        ");
    let a_val = "1c0111001f010100061a024b53535009181c";
    let b_val = "686974207468652062756c6c277320657965";
    let a_bytes: Vec<u32> = a_val.chars().map(|x| x.to_digit(16).unwrap()).collect();
    let b_bytes: Vec<u32> = b_val.chars().map(|x| x.to_digit(16).unwrap()).collect();

    // let a_bytes: Vec<u32> = encoders::hex_str_to_hex_val(a_val);
    // let b_bytes: Vec<u32> = encoders::hex_str_to_hex_val(b_val);

    let result = xor::helpers::xor_same_size_buffers(a_bytes, b_bytes);
    let result_str: String = result.iter().map(|x| translators::to_hex(&x)).collect();
    
    dbg!(assert_eq!(result_str, "746865206b696420646f6e277420706c6179"));
    assert_eq!(result_str, "746865206b696420646f6e277420706c6179");
}