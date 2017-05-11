extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::distributions::Range;

const FILENAME: &str = "../../resources/big_list_of_crap.txt";
const FORMAT_CHAR: u8 = b'%';

fn main() {
    let nouns: Vec<String> = read_from_file;
    let a_template: Template = Template { text: "I love %." };
    println!("{}", a_template.format(&nouns));
}

fn read_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("couldn't parse line")).collect()
}

struct Template {
    text: &'static str,
}

impl Template {
    fn format(&self, nouns: &Vec<String>) -> String {
        let mut new_bytes: Vec<u8> = Vec::new();
        for byte in self.text.bytes() {
            if byte == FORMAT_CHAR {
                let ref noun_phrase = nouns[0];
                for noun_byte in noun_phrase.bytes() {
                    new_bytes.push(noun_byte);
                }
            } else {
                new_bytes.push(byte);
            }
        }
        return String::from_utf8(new_bytes).unwrap();
    }
}
