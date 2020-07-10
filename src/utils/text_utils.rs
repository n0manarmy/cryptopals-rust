use std::collections::BTreeMap;

pub fn hamming_distance(buf1: Vec<char>, buf2: Vec<char>) -> usize {

    0
}

pub fn ascii_scoring(buf: &Vec<char>) -> (f32, BTreeMap<String, f32>) {
    let mut ascii_map: BTreeMap<String, f32> = BTreeMap::new();
    for b in buf {
        if !ascii_map.contains_key(&b.to_string()) {
            ascii_map.insert(b.to_string(), scoring(*b));
        } else {
            ascii_map.insert(b.to_string(), ascii_map.get_key_value(&b.to_string()).unwrap().1 + scoring(*b));
        }
    }
    let mut score = 0.;
    for (_key, val) in ascii_map.iter() {
        score = score + val;
    }
    (score, ascii_map)
}

fn scoring(c: char) -> f32 {
    // let lower: String = c.to_lowercase().to_string();
    match c.to_string().as_ref() {
        "e" => 0.11162,
        "t" => 0.09365,
        "a" => 0.08497,
        "r" => 0.07587,
        "i" => 0.07564,
        "o" => 0.07507,
        "n" => 0.06749,
        "s" => 0.06327,
        "h" => 0.06094,
        "d" => 0.04253,
        "l" => 0.04025,
        "u" => 0.02758,
        _ => 0.0,
    }
}

pub fn non_standard_single_letter(val: &Vec<char>) -> bool {
    let collected:String = val.iter().collect();
    let alpha = "bcdefghjklmnpqrstuvwxyz";
    for a in alpha.chars() {
        let mut pattern_lower = String::new();
        pattern_lower.push_str(" ");
        pattern_lower.push(a);
        pattern_lower.push_str(" ");

        let mut pattern_upper = String::new();
        pattern_upper.push_str(" ");
        pattern_upper.push(a.to_ascii_uppercase());
        pattern_upper.push_str(" ");


        if collected.contains(&pattern_lower) || collected.contains(&pattern_upper){
            return true;
        }
    }

    false
}

pub fn count_spaces(val: &Vec<char>) -> u32 {
    let mut sum = 0;
    for v in val {
        if *v == ' ' {
            sum += 1;
        }
    }

    sum
}

pub fn garbage_found(val: &Vec<char>) -> bool {
    for v in val {
        match v {
            '$' |
            '%' |
            '&' |
            '{' |
            '}' |
            '#' |
            '~' |
            'ƒ' |
            '„' |
            '…' |
            '†' |
            '‡' |
            'ˆ' |
            '‰' |
            'Š' |
            '‹' |
            'Œ' |
            '”' |
            '–' |
            '—' |
            '˜' |
            '™' |
            'š' |
            '›' |
            'œ' |
            'Ÿ' |
            '¡' |
            '¢' |
            '£' |
            '¤' |
            '¥' |
            '¦' |
            '§' |
            '¨' |
            '©' |
            'ª' |
            '«' |
            '¬' |
            '®' |
            '¯' |
            '°' |
            '±' |
            '²' |
            '³' |
            '´' |
            'µ' |
            '¶' |
            '·' |
            '¹' |
            'º' |
            '»' |
            '¼' |
            '½' |
            '¾' |
            '¿' |
            'À' |
            'Á' |
            'Â' |
            'Ã' |
            'Ä' |
            'Å' |
            'Æ' |
            'Ç' |
            'È' |
            'É' |
            'Ê' |
            'Ë' |
            'Ì' |
            'Í' |
            'Î' |
            'Ï' |
            'Ð' |
            'Ñ' |
            'Ò' |
            'Ó' |
            'Ô' |
            'Õ' |
            'Ö' |
            '×' |
            'Ø' |
            'Ù' |
            'Ú' |
            'Û' |
            'Ü' |
            'Ý' |
            'Þ' |
            'ß' |
            'à' |
            'á' |
            'â' |
            'ã' |
            'ä' |
            'å' |
            'æ' |
            'ç' |
            'è' |
            'é' |
            'ê' |
            'ë' |
            'ì' |
            'í' |
            'î' |
            'ï' |
            'ð' |
            'ñ' |
            'ò' |
            'ó' |
            'ô' |
            'õ' |
            'ö' |
            '÷' |
            'ø' |
            'ù' |
            'ú' |
            'û' |
            'ü' |
            'ý' |
            'þ' |
            'ÿ' |
            '[' |
            ']' |
            '|' |
            '@' => return true,
            _   =>   continue,
        }
    }
    
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_chars() {
        let val = vec!['¾'];
        assert_eq!(garbage_found(&val), true);
    }

    #[test]
    pub fn test_hamming_distance() {
        let val1_str = "this is a test";
        let val2_str = "wokka wokka!!!";
        let val1_buf: Vec<char> = val1_str.chars().collect();
        let val2_buf: Vec<char> = val2_str.chars().collect();

        assert_eq!(hamming_distance(val1_buf, val2_buf), 0);
    }

}