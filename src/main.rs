extern crate rand;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::{Rng, thread_rng, ThreadRng};

const NOUNS_FILENAME: &str = "resources/nouns.txt";
const TEMPLATE_FILENAME: &str = "resources/templates.txt";

mod template;
use template::Template;

fn main() {
    let nouns: Vec<String> = read_nouns_from_file(NOUNS_FILENAME);
    let templates: Vec<Template> = Template::load_from_file(TEMPLATE_FILENAME);
    let mut rng = thread_rng();
    println!("{}", choose_and_format(&templates, &nouns, &mut rng));
}

fn read_nouns_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("couldn't parse line")).collect()
}

fn choose_and_format(templates: &Vec<Template>, nouns: &Vec<String>, mut rng: &mut ThreadRng) -> String {
    if let Some(ref template) = rng.choose(&templates) {
        template.format(&nouns, &mut rng)
    } else {
        panic!("couldn't pick a template")
    }
}
