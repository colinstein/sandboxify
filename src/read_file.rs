use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut file = File::open(filename).expect("no such file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("could not read file");
    println!("{}", content);
}
