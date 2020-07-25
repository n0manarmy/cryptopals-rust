use crate::aes::key_expander::helper::{ek, rot_word, rcon, collect_to_vec, k};
use crate::aes;

pub fn expand(key: &Vec<u8>) -> Vec<u8> {

    //block size is always 16
    const BLOCK_SIZE: usize = 16;
    
    //expand the expanded key to be the key_size * block_size + 1 per specs
    let r_setup = match key.len() {
        16  => (44, 4),
        24  => (52, 6),
        32  => (60, 8),
        _ => panic!("Error in expand. Key_size non standard"),
    };

    let mut expanded_key: Vec<u8> = Vec::with_capacity((key.len() * BLOCK_SIZE) + 1);

    // start key expansion
    let mut x = 0;
    while x < r_setup.0 {
        if x < r_setup.1 {
            // println!("xs: {} - ", x);
            let mut sub_key: Vec<u8> = k(x * 4, &key);
            expanded_key.append(&mut sub_key);
            x += 1;
            continue;
        }
        
        //sub word application
        // print!("xo: {} - ", x);
        let ek_first = ek((x - 1) * 4, &expanded_key);
        // print!("ekf: {:08x} ", ek_first);
        let ek_first = rot_word(ek_first);
        // print!("rot: {:08x} ", ek_first);
        let ek_first = aes::helper::byte_sub(ek_first);
        // print!("sub: {:08x} ", ek_first);
        let rconned = rcon(x, key.len());
        // print!("rcn: {:08x} ", rconned);
        let ek_second = ek(((x - 4) * 4).into(), &expanded_key);
        // print!("eks: {:08x} ", ek_second);
        let xord = ek_first ^ ek_second ^ rconned;
        // print!("xrd: {:08x} ", xord);
        let mut xord = collect_to_vec(xord);
        expanded_key.append(&mut xord);
        // print!("len: {} ", (expanded_key.len() /4) - 1);
        x += 1;
        println!();
        
        let mut sub = 3;
        if r_setup.0 == 52 {
            sub = 5;
        }
        
        for _z in 0..sub {
            // 192bit keys finish with 3 rounds instead of standard 5 rounds for 192bit.
            if sub == 5 && x == 52 {
                break;
            }
            // 240bit keys finish with 2 rounds instead of standard 3 rounds for 256bit
            if sub == 3 && x == 60 {
                break;
            }
            // print!("xi: {} - ", x);
            let a = ek((x - 1) * 4, &expanded_key);
            // print!("a: {:08x} ", a);
            let b = ek((x - 4) * 4, &expanded_key);
            // print!("b: {:08x} ", b);
            expanded_key.append(&mut collect_to_vec(a ^ b));
            // print!("len: {} ", (expanded_key.len() /4) - 1);
            println!();
            x += 1;
        }

    }

    expanded_key
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_expand_128() {
        // let key: Vec<u8> = String::from("1234567890111213").bytes().collect();
        let key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let expanded = expand(&key);
        assert_eq!(expanded.len(), (44 * 4));
        
        for x in 0..expanded.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02x}", expanded[x]);
        }
    }

    #[test]
    pub fn test_expand_192() {
        // let key: Vec<u8> = String::from("1234567890111213").bytes().collect();
        let key: Vec<u8> = vec![
            0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 
            0x80, 0x90, 0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b];
        let expanded = expand(&key);
        assert_eq!(expanded.len(), (52 * 4));
        
        for x in 0..expanded.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02x}", expanded[x]);
        }
    }

    #[test]
    pub fn test_expand_256() {
        // let key: Vec<u8> = String::from("1234567890111213").bytes().collect();
        let key: Vec<u8> = vec![
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81, 
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4];
        let expanded = expand(&key);
        assert_eq!(expanded.len(), (60 * 4));
        
        for x in 0..expanded.len() {
            if x % 4 == 0 {
                println!();
                print!("{} ", (x/4) + 1);
            }
            print!("{:02x}", expanded[x]);
        }
    }
}