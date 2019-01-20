use std::env;

use rand::Rng;

fn main() {
    let input: usize = match env::args().nth(1) {
        Some(args1) => args1.parse().unwrap(),
        None => panic!("missing argument"),
    };

    let mut rng = rand::thread_rng();
    let mut chars: Vec<char>=Vec::new();
    //generate
    for mut _i in 0..input {
        let rnd: u8 = rng.gen_range(32, 127);
        chars.push(rnd as char);
    }
    let str: String = chars.iter().collect();
    println!("{}", str);
}
