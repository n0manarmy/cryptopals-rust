extern crate cryptopals_lib;
extern crate rusty_aes;
use std::cmp::Ordering;
use crate::cryptopals_lib::utils::file_io_utils;
use crate::rusty_aes::decrypt::Decrypt;
use crate::cryptopals_lib::hex;

pub fn main() {
    let file_name = "challenge_files/8.txt";
    
    //read file to string
    let input = file_io_utils::read_file_by_lines_to_vec_str(&file_name);
    let mut t_input: Vec<Vec<u8>> = Vec::new();
    for i in input {
        t_input.push(hex::encoders::str_to_hex_u8_buf(&i));
    }

    // let key = "YELLOW SUBMARINE".as_bytes().to_vec();
    
    //instantiate our aes decryptor
    // for (i, t) in t_input.iter().enumerate() {
    //     println!("index: {} count: {}", i, pattern_match(t, 4));
    // }

    let collected = collect_chunks(&t_input);
    let mut collected = chunk_match(collected, &t_input);

    collected.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap_or(Ordering::Equal));

    for c in collected {
        if c.count > 1 {
            dbg!(c);
        }
    }


    // let decrypt: Decrypt = Decrypt::new(key);

    // for t in t_input {
    //     process(t, &decrypt);
    // }

}

#[derive(Debug)]
pub struct Chunk {
    line_pos: usize,
    count: usize,
    chunk: Vec<u8>,
}

fn chunk_match(mut chunks: Vec<Chunk>, input: &Vec<Vec<u8>>) -> Vec<Chunk> {
    for mut c in chunks.iter_mut() {
        for i in input {
            let mut pos = 0;
            while pos < i.len() {
                if c.chunk.iter().zip(i[pos..pos + 15].to_vec()).all(|(a,b)| eq_with_nan_eq(*a, b)) {
                    c.count += 1;
                }
                pos += 16;
            }
        }
    }

    chunks
}

fn collect_chunks(input: &Vec<Vec<u8>>) -> Vec<Chunk> {
    let mut collected: Vec<Chunk> = Vec::new();
    for (i, v) in input.iter().enumerate() {
        let mut pos = 0;
        while pos < v.len() {
            collected.push(Chunk {line_pos: i + 1, count: 0, chunk: v[pos..pos + 15].to_vec()});
            pos += 16;
        }
    }

    collected
}

fn eq_with_nan_eq(a: u8, b: u8) -> bool {
    a == b
}

fn pattern_match(line: &Vec<u8>, check_size: usize) -> usize {
    assert_eq!(line.len() % 16, 0);
    let runs = line.len() / check_size;
    let mut count = 0;
    let mut index = 0;
    for _x in 0..runs {
        let index_val: Vec<u8> = line[index..index + check_size].to_vec();
        // for y in (0..line.len()).step_by(check_size) {
        for y in 0..line.len() {
            if y + check_size <= line.len() {
                if index_val.iter().zip(line[y..y + check_size].to_vec()).all(|(a,b)| eq_with_nan_eq(*a, b)) {
                    count += 1;
                }
            }
        }
        index += check_size;
    }

    count
}

fn process(input: Vec<u8>, decrypt: &Decrypt) {
    let mut buf: Vec<u8> = Vec::new();
    let buf_len = 16;
    println!();
    let mut count = 0;
    while count < input.len() {
        if count + buf_len >= input.len() {
            // buf = enc_str[count..(enc_str.len() - count)].to_vec();
            let mut slice = input[count..count + (input.len() - count)].to_vec();
            let padding = buf_len - slice.len() ;
            for _z in 0..padding {
                slice.push(0x80);
            }
            buf.append(&mut decrypt.decrypt(slice));
        } 
        else {
            let slice = input[count..(count + buf_len)].to_vec();
            assert_eq!(slice.len(), buf_len);
            buf.append(&mut decrypt.decrypt(slice));
        }
        count += buf_len;
    }
    for b in buf {
        print!("{}", b as char);
    }
    println!();
}

#[cfg(test)]
mod tests {

}