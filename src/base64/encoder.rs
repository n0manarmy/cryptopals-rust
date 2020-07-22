use crate::utils::translators;

fn encode_hex_vec(buf: Vec<u32>) -> String {
    let mut buf_size = 0;
    let mut byte_buf: u32 = 0;
    let mut encoded: String = String::new();

    for b in buf {
        buf_size += 1;
        byte_buf |= b;

        if buf_size == 6 {
            let encoding = encode_match(byte_buf, buf_size/2);
            // dbg!(&encoding);
            encoded.push_str(&encoding);
            byte_buf = 0;
            buf_size = 0;
            continue
        }

        byte_buf <<= 4;
    }

    if buf_size != 0 {
        // dbg!(byte_buf);
        let encoding = encode_match(byte_buf, buf_size/2);
        encoded.push_str(&encoding);
    }

    encoded
}


pub fn encode_hex_str(buf: &'static str) -> String {
    let mut buf_size = 0;
    let mut byte_buf: u32 = 0;
    let mut encoded: String = String::new();

    for c in buf.chars() {
        buf_size += 1;
        byte_buf |= c.to_digit(16).unwrap();

        if buf_size == 6 {
            let encoding = encode_match(byte_buf, buf_size/2);
            encoded.push_str(&encoding);
            byte_buf = 0;
            buf_size = 0;
            continue
        }

        byte_buf <<= 4;
    }

    if buf_size != 0 {
        let encoding = encode_match(byte_buf, buf_size/2);
        encoded.push_str(&encoding);
    }

    encoded
}

fn encode_str(val: &str) -> String {
    let mut buf_size: u8 = 0;
    let mut encoded: String = String::new();
    let mut byte_buf: u32 = 0;
    
    for c in val.chars()  {
        buf_size += 1;
        byte_buf |= c as u32;

        //byte buffer full, encode to base64
        if buf_size == 3 { 
            let encoding = encode_match(byte_buf, buf_size);
            encoded.push_str(&encoding);
            byte_buf = 0;
            buf_size = 0;
            continue;
        }
        
        byte_buf <<= 8;
    }

    if buf_size != 0 {
        let encoding = encode_match(byte_buf, buf_size);
        encoded.push_str(&encoding);
    }

    encoded
}


fn encode_match(mut byte_buf: u32, buf_size: u8) -> String {
    let mut encoded: String = String::new();
    let mut pos = 0;
    let base64_buf = match buf_size {
        1 => {byte_buf <<= 8; 2},
        2 => 3,
        3 => 4,
        _ => panic!("Error processing base64_buff")
    };

    while pos < base64_buf {
        let b = (byte_buf & 0xFC0000) >> 18;
        let val = translators::to_base64(b);
        byte_buf <<= 6;
        encoded.push(val);
        pos += 1;
    }

    match buf_size {
        1 => {encoded.push('=');encoded.push('=')},
        2 => encoded.push('='),
        _ => (),
    }

    encoded
}

#[cfg(test)]
mod tests {

    use super::*;

    const VAL: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const RESULT: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
    const T1: &'static str = "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.";
    const T1_ANSWER: &'static str = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

    #[test]
    pub fn test_alpha_to_hex() {
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        for a in alpha.chars() {
            println!("{}", a);
        }
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
    pub fn test_base64_match() {
        let input: u32 = 0b0100_1101_0110_0001_0110_1110;
        assert_eq!(encode_match(input, 3), "TWFu");
    }

    #[test]
    pub fn test_hex_decode() {
        assert_eq!(encode_hex_str(VAL), RESULT);
    }

    #[test]
    pub fn test_manual_encode() {
        dbg!(encode_match(0b_00100000_01100100, 2));
    }

    #[test]
    pub fn test_run() {
        assert_eq!(encode_hex_str(VAL), RESULT);
        assert_eq!(encode_str(T1), T1_ANSWER);
    }

    #[test]
    pub fn test_run_2() {
        let test_val = ("s", "cw==");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Ma", "TWE=");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Man", "TWFu");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Man is", "TWFuIGlz");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Man is d", "TWFuIGlzIGQ=");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Man is di", "TWFuIGlzIGRp");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
        let test_val = ("Man is dis", "TWFuIGlzIGRpcw==");
        dbg!(test_val);
        assert_eq!(encode_str(test_val.0),test_val.1);
    }
}