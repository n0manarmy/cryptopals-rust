use crate::utils::translators;
use std::char;


pub fn decode_str(val: &str) -> String {
    let mut decoded: String = String::new();
    let mut byte_buf: u32 = 0;
    let mut buf_size: usize = 0;

    let padding = val.len() - val.trim_end_matches('=').len();
    let val = val.trim_end_matches('=');

    //cw==
    for c in val.chars() {
        buf_size += 1;
        // dbg!(c);
        byte_buf |= translators::from_base64(c) as u32;
        // println!("1: {:b} buf_size: {}", byte_buf, buf_size);
        if buf_size == 4 {
            let bytes = decode_match(byte_buf, buf_size);
            // println!("inside: {}", bytes);
            decoded.push_str(&bytes);
            buf_size = 0;
        }
        byte_buf <<= 6;
    }
    // println!("2: {:b} buf_size: {}", byte_buf, buf_size);

    if padding == 2 {
        let bytes = decode_match(byte_buf >> 10, buf_size);
        decoded.push_str(&bytes);
    }
    
    if padding == 1 {
        let bytes = decode_match(byte_buf >> 8, buf_size);
        decoded.push_str(&bytes);
    }

    decoded
}


fn decode_match(byte_buf: u32, mut buf_size: usize) -> String {
    let mut str_buff: String = String::new();
    // println!("decode_match -- byte_buf: {:b} buf_size: {}", byte_buf, buf_size);
    while buf_size > 0 {
        let byte = match buf_size {
            4 => ((byte_buf & 0xFF0000) >> 16) as u8,
            3 => ((byte_buf & 0xFF00) >> 8) as u8,
            2 => (byte_buf & 0xFF) as u8,
            1 => break,
            _ => panic!("Error in decode_match, unknown value found"),
        };

        buf_size -= 1;
        // println!("byte after match {:b} {}", byte, byte);
        str_buff.push(byte as char);
        // dbg!(&str_buff);
    }
    str_buff
}

#[cfg(test)]
mod tests {

    use super::*;

    const ASCII: &'static str = "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.";
    const BASE64: &'static str = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

    #[test]
    pub fn test_alpha_to_hex() {
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        for a in alpha.chars() {
            println!("{}", a);
        }
    }

    #[test]
    pub fn test_base64_to_ascii() {
        assert_eq!(decode_str(BASE64), ASCII);
    }

    #[test]
    pub fn test_xor_values() {
        let val = "abcdefg"; // a
        let alpha = 'a'; // b
        dbg!(alpha as u32);
        println!("alpha as u32 as hex {:x}", alpha as u32);
        println!("alpha to_digit as hex {:x}", alpha.to_digit(16).unwrap());
        println!("alpha to_digit as hex {:b}", alpha.to_digit(16).unwrap());
        dbg!(translators::ascii_to_hex(&alpha.to_string()));
        let hex_enc_val: Vec<u32> = val.chars().map(|x| x as u32).collect(); // convert to hex
        dbg!(&hex_enc_val);
        let xor_env_val: Vec<u32> = val.chars().map(|x| x as u32 ^ alpha as u32).collect(); // xor a ^ b = c
        dbg!(&xor_env_val);
        let re_xor_env_val: Vec<u32> = xor_env_val.iter().map(|x| *x ^ alpha as u32).collect(); // xor c ^ b = a
        dbg!(&re_xor_env_val);
        let decoded: Vec<&str> = re_xor_env_val.iter().map(|x| {
            translators::hex_val_to_ascii(*x).1
        }).collect();
        dbg!(decoded);
    }

    #[test]
    pub fn test_xor_single() {
        let val = "1b37";
        for v in val.chars() {
            print!("|char {} ", v);
            print!("|as u32 {} ", v as u32);
            print!("|to_digit(16) {} ", v.to_digit(16).unwrap());
            println!();
        }
    }
    

    #[test]
    pub fn test_run_2() {
        let test_val = ("s", "cw==");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Ma", "TWE=");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Man", "TWFu");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Man is", "TWFuIGlz");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Man is d", "TWFuIGlzIGQ=");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Man is di", "TWFuIGlzIGRp");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
        let test_val = ("Man is dis", "TWFuIGlzIGRpcw==");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
    }

    #[test]
    pub fn test_decode() {
        let test_val = ("s", "cw==");
        dbg!(test_val);
        assert_eq!(decode_str(test_val.1),test_val.0);
    }

    #[test]
    pub fn test_trim_count() {
        let x = "cw==";
        let y = x.trim_end_matches('=');
        assert_eq!(y.len(), x.len() -2);
    }
}