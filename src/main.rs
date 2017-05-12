extern crate rand;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::{Rng, thread_rng, ThreadRng};

const NOUNS_FILENAME: &str = "resources/nouns.txt";
const TEMPLATE_FILENAME: &str = "resources/templates.txt";
const ADJECTIVES_FILENAME: &str = "resources/adjectives.txt";
const ADVERBS_FILENAME: &str = "resources/adverbs.txt";

mod template_manager;
use template_manager::Template_manager;

fn main() {
    let manager: Template_manager = Template_manager::new(TEMPLATE_FILENAME, NOUNS_FILENAME, ADJECTIVES_FILENAME, ADVERBS_FILENAME);
    let mut rng: ThreadRng = thread_rng();
    println!("{}", manager.make_formatted_quote(&mut rng));
}
