use std::collections::BTreeMap;
use std::io::prelude::*;


pub fn read_file(f: &str) -> String {
    let file = std::path::Path::new(f);

    let contents = match std::fs::read_to_string(&file) {
        Ok(v) => v,
        Err(why) => panic!(why.to_string()),
    };

    contents

}

pub fn read_file_by_lines(f: &str) -> Vec<String> {
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
    pub fn test_read_file() {
        let results = read_file("challenge_files/4.txt");
        assert_eq!(results.contains("0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032"), true);
    }
}