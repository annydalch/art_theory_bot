use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub struct Phrase {
    // a phrase is a string that uses %, @, and # as wildcards for other phrases
    pub text: String,
}

impl Phrase {
    pub fn load_from_file<P>(filename: P) -> Vec<Phrase>
        where P: AsRef<Path>
    // given the path to one of the big lists of phrases, load the phrases as a Vec<Phrase>
    // we do Vec<Phrase> because the Rng.choose() method is value af
    {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines().map(|l| Phrase { text: l.expect("couldn't parse line") }).collect()
    }
}
