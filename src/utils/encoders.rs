use super::*;

pub fn str_to_hex_val(buf: String) -> Vec<u32> {
    let mut hex_buf: Vec<u32> = Vec::new();
    
    let mut hex_merge: u32 = 0;
    let mut pos = 1;
    
    for c in buf.chars() {
        hex_merge |= c.to_digit(16).unwrap();

        if pos == 2 {
            hex_buf.push(hex_merge);
            hex_merge = 0;
            pos = 1;
            continue;
        }

        pos += 1;
        hex_merge <<= 4;
    }
    
    hex_buf
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

}