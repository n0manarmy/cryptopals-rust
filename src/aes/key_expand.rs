pub fn expand(key: Vec<u8>) -> Vec<u8> {

    //block size is always 16
    const BLOCK_SIZE: usize = 16;
    
    //expand the expanded key to be the key_size * block_size + 1 per specs
    let mut expanded_key: Vec<u8> = match key.len() {
        16 | 24 | 32 => Vec::with_capacity((key.len() * BLOCK_SIZE) + 1),
        _ => panic!("Error in expand. Key_size non standard"),
    };

    //start key expansion
    for x in 0..expanded_key.len() {
        if x < 4 {
            let mut expansion = k(x, &key);
            expanded_key.append(&mut expansion);
        }

    }

    expanded_key
}

fn rot_word(bytes: &mut Vec<u8>) {
    let temp = bytes[0];
    for x in 0..bytes.len() {
        if x + 1 == 4 {
            bytes[x] = temp;
            break;
        }
        bytes[x] = bytes[x + 1];
    }
}

fn sub_word() {}

fn rcon(rounds: usize, key_size: usize) -> u32 {
    let lookup = (rounds/(key_size/4)) - 1;
    match lookup {
        0   => 0x01000000, 
        1   => 0x02000000, 
        2   => 0x04000000, 
        3   => 0x08000000, 
        4   => 0x10000000, 
        5   => 0x20000000, 
        6   => 0x40000000, 
        7   => 0x80000000, 
        8   => 0x1B000000, 
        9   => 0x36000000, 
        10  => 0x6C000000, 
        11  => 0xD8000000, 
        12  => 0xAB000000, 
        13  => 0x4D000000, 
        14  => 0x9A000000,
        _   => panic!("Error in rcon lookup table"),
    }

}

fn ek(offset: usize, expanded_key: &Vec<u8>) {
    vec![expanded_key[offset], expanded_key[offset + 1], expanded_key[offset + 2], expanded_key[offset + 3]];
}

fn k(offset: usize, key: &Vec<u8>) -> Vec<u8> {
    vec![key[offset], key[offset + 1], key[offset + 2], key[offset + 3]]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_expand() {
        let key: Vec<u8> = String::from("1234567890111213").bytes().collect();
        let expanded = expand(key);
    }

    #[test]
    pub fn test_rot_word() {
        let mut key: Vec<u8> = vec![1, 2, 3, 4];
        rot_word(&mut key);
        dbg!(key);
    }

    #[test]
    pub fn test_rcon() {
        let rounds = 4;
        let key_size = 16;
        assert_eq!(rcon(rounds, key_size), 0x01000000);
    }
}