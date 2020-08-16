extern crate rusty_aes;
use crate::rusty_aes::utils::padder;

/* 
Implement PKCS#7 padding

A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext.
But we almost never want to transform a single block; we encrypt irregularly-sized messages.

One way we account for irregularly-sized messages is by padding, creating a plaintext that is an 
even multiple of the blocksize. The most popular padding scheme is called PKCS#7.

So: pad any block to a specific block length, by appending the number of bytes of padding to 
the end of the block. For instance,

"YELLOW SUBMARINE"

... padded to 20 bytes would be:

"YELLOW SUBMARINE\x04\x04\x04\x04"
*/

fn main() {
    let key: Vec<u8> = "YELLOW SUBMARINE".chars().map(|x| x as u8).collect();
    let mut pad_test: Vec<u8> = "YELLOW SUBMARINE".chars().map(|x| x as u8).collect();
    pad_test.append(&mut vec![0x80, 0x80, 0x80, 0x80]);
    assert_eq!(padder::pad(key, 20), pad_test);
}