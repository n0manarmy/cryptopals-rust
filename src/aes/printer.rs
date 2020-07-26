pub fn print_state(state: &Vec<u8>) {
    const VERT: u8 = 1;
    let mut horz = 0;
    // horz = VERT;
    if horz == 0 {
        for x in 0..state.len() {
            if x % 4 == 0 {
                println!();
                print!("{:02} - ", (x/4 + 1));
            }
            print!("{:02x} ", state[x]);
        }
    } else {
        for x in 0..state.len() {
            if x % 4 == 0 {
                print!(" ");
                // print!("{:02} ", (x/4) + 1);
            }
            if x % 16 == 0 {
                println!();
                // print!("{:02} ", (x/4) + 1);
            }
            print!("{:02x}", state[x]);
        }
    }
    println!();
}