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

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_alpha_to_hex() {
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        for a in alpha.chars() {
            println!("{}", a);
        }
    }
}