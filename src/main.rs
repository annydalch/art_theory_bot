use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const FILENAME: &str = "../../resources/big_list_of_crap.txt";

fn main() {
    read_from_file(FILENAME);
}

fn read_from_file(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    println!("{}", contents);
}
