use std::char;

pub fn main() {
    println!("

    Set 1 Challenge 5
    
    Implement repeating-key XOR

    Here is the opening stanza of an important work of the English language:

    Burning 'em, if you ain't quick and nimble
    I go crazy when I hear a cymbal

    Encrypt it, under the key \"ICE\", using repeating-key XOR.

    In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.

    It should come out to:

    0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
    a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

    Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.

    ");

    let msg: Vec<&str> = vec!["Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"];
    let key: Vec<char> = vec!['I', 'C', 'E'];
    let answers = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let mut enc_buff: Vec<u8> = Vec::new();
    // let msg_hex = encoders::str_to_hex_val(msg.to_string());

    for m in msg {
        let mut pos = 0;
        for c in m.chars() {
            if pos == 3 {
                pos = 0;
            }
            enc_buff.push(c as u8 ^ key[pos] as u8);
            pos += 1;
        }
    }

    let enc_buff = enc_buff.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    assert_eq!(enc_buff, answers);
    println!("Enc buff:\t{}\nAnswer:\t\t{}", enc_buff, answers);
    // let checker_iter = enc_buff.chars().zip(answers.chars());

    // let mut pos = 0;
    // for (m, a) in checker_iter {
    //     println!("pos: {}, m: {} -- a: {}", pos, m, a);
    //     assert_eq!(m, a);
    //     pos += 1;
    // }
}