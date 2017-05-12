extern crate rand;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::{Rng, thread_rng, ThreadRng};

const FORMAT_CHAR: u8 = b'%';

pub struct Template {
    text: String,
}

impl Template {
    pub fn format(&self, nouns: &Vec<String>, mut rng: &mut ThreadRng) -> String {
        let mut new_bytes: Vec<u8> = Vec::new();
        for byte in self.text.bytes() {
            if byte == FORMAT_CHAR {
                if let Some(ref noun_phrase) = rng.choose(&nouns) {
                    for noun_byte in noun_phrase.bytes() {
                        new_bytes.push(noun_byte);
                    }
                } else {
                    panic!("couldn't pick a noun phrase");
                }
            } else {
                new_bytes.push(byte);
            }
        }
        return String::from_utf8(new_bytes).unwrap();
    }
    pub fn load_from_file<P>(filename: P) -> Vec<Template>
        where P: AsRef<Path>
    {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines().map(|l| Template { text: l.expect("couldn't parse line") }).collect()
    }
}
