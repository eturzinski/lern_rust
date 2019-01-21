use std::env;

use rand::Rng;

fn main() {
    let input: usize = env::args()
        .nth(1)
        .expect("missing argument")
        .parse()
        .expect("argument needs to be a positive number");
    let mut string: String = String::with_capacity(input);
    let mut rng = rand::thread_rng();
    for mut _i in 0..input {
        let rnd: u8 = rng.gen_range(32, 127);
        string.push(rnd as char);
    }
    println!("{}", string);
}
