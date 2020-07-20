use std::io::prelude::*;

pub fn collect_bytes(count: usize, mut pos: usize, bytes: &Vec<u8>) -> Vec<u8> {
    let mut new_vec: Vec<u8> = Vec::new();
    for _x in 0..count {
        new_vec.push(bytes[pos]);
        pos += 1;
    }

    new_vec
}
pub fn read_file_to_bytes(f: &str) -> Vec<u8>{
    let file = std::path::Path::new(f);

    let contents = match std::fs::read(&file) {
        Ok(v) => v,
        Err(why) => panic!(why.to_string()),
    };

    contents
}

pub fn read_file(f: &str) -> String {
    let file = std::path::Path::new(f);

    let contents = match std::fs::read_to_string(&file) {
        Ok(v) => v,
        Err(why) => panic!(why.to_string()),
    };

    contents
}

pub fn hamming_distance(buf1: &Vec<u8>, buf2: &Vec<u8>) -> u32 {
    let iter = buf1.iter().zip(buf2.iter());

    iter.map(|(x, y)| (x ^ y).count_ones()).sum()

}

pub fn read_file_by_lines_to_str(f: &str) -> String {
    let f_reader = match std::fs::File::open(std::path::Path::new(f)) {
        Ok(f) => f,
        Err(why) => panic!(why.to_string()),
    };

    let f_reader = std::io::BufReader::new(f_reader);
    let mut f_buf: String = String::new();

    for line in f_reader.lines() {
        match line {
            Ok(l) => f_buf.push_str(l.trim()),
            Err(why) => panic!(why.to_string()),
        };
    }

    f_buf


}

pub fn read_file_by_lines_to_vec(f: &str) -> Vec<String> {
    let f_reader = match std::fs::File::open(std::path::Path::new(f)) {
        Ok(f) => f,
        Err(why) => panic!(why.to_string()),
    };

    let f_reader = std::io::BufReader::new(f_reader);
    let mut f_buf: Vec<String> = Vec::new();

    for line in f_reader.lines() {
        match line {
            Ok(l) => f_buf.push(l),
            Err(why) => panic!(why.to_string()),
        };
    }

    f_buf


}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test_hamming_distance() {
        let val1_str = "this is a test";
        let val2_str = "wokka wokka!!!";
        // let val1_buf: Vec<u8> = val1_str.bytes().collect();
        let val1_buf: Vec<u8> = collect_bytes(14, 0, &val1_str.bytes().collect());
        // let val2_buf: Vec<u8> = val2_str.bytes().collect();
        let val2_buf: Vec<u8> = collect_bytes(14, 0, &val2_str.bytes().collect());
        let results = hamming_distance(&val1_buf, &val2_buf);

        dbg!(val1_str, val2_str, results, 37);
        assert_eq!(results, 37);

        let val1_str = "this is a tes";
        let val2_str = "wokka wokka!!!";
        let val1_buf: Vec<u8> = val1_str.bytes().collect();
        let val2_buf: Vec<u8> = val2_str.bytes().collect();
        let results = hamming_distance(&val1_buf, &val2_buf);

        dbg!(val1_str, val2_str, results, 37);
        assert_ne!(results, 37);


    }

    #[test]
    pub fn test_read_file() {
        let results = read_file("challenge_files/4.txt");
        assert_eq!(results.contains("0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032"), true);
    }
}