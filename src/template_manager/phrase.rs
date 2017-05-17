use std::io::BufReader;

// i'm not sure what std::io::prelude is and the docs aren't helpful
// but it was in all the read from file examples
// and rustc doesn't warn about unused imports
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::Result;

pub struct Phrase {
    // a phrase is a string that uses %, @, and # as wildcards for other phrases
    pub text: String,
}

impl Phrase {
    pub fn load_from_file<P>(filename: P) -> Result<Vec<Phrase>>
        where P: AsRef<Path>
    // given the path to one of the big lists of phrases, load the phrases as a Vec<Phrase>
    // we do Vec<Phrase> because the Rng.choose() method is value af
    {
        let file = File::open(filename)?;
        let buf = BufReader::new(file);
        let mut lines: Vec<Phrase> = Vec::new();
        for line in buf.lines() {
            match line {
                Ok(l) => {
                    lines.push( Phrase { text: l } );
                }
                Err(err) => {return Err(err);},
            }
        }
        Ok(lines)
    }
}
