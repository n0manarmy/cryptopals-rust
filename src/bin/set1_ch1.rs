extern crate cryptopals_lib;
use crate::cryptopals_lib::base64::{encoder, decoder};

pub fn main() {
    println!(
        "
        Chapter 1 Challenge 1

        Convert hex to base64
        The string:

        49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d

        Should produce:
        SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

        So go ahead and make that happen. You'll need to use this code for the rest of the exercises.

        Cryptopals Rule
        Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.
    
        ");
    let val = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
    dbg!(assert_eq!(encoder::encode_hex_str(val), result));
    assert_eq!(encoder::encode_hex_str(val), result);

    let decode = decoder::decode_str(result);
    println!("decoded {}", &decode);
}