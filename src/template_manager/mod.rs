extern crate rand;

use std::io::BufReader;
use std::io::prelude::*; // I'm honestly not sure what this is but all the examples I found online included it
use std::fs::File;
use std::path::Path;
use rand::{Rng, thread_rng, ThreadRng};

const NOUN_FORMAT_CHAR: u8 = b'%';
const ADJECTIVE_FORMAT_CHAR: u8 = b'#';
const ADVERB_FORMAT_CHAR: u8 = b'@';

mod phrase;
use self::phrase::Phrase;
// Phrase is just a wrapper around a String with a method to load a Vec of them from a file

pub struct Template_manager {
    // this is a struct that handles all of the actual text generation
    // we use Vec<Phrase> so we can use Rng.choose()
    templates: Vec<Phrase>,
    nouns: Vec<Phrase>,
    adjectives: Vec<Phrase>,
    adverbs: Vec<Phrase>,
}


impl Template_manager {
    pub fn new<P0, P1, P2, P3>(template_path: P0, noun_path: P1, adj_path: P2, adv_path: P3) -> Template_manager
        where P0: AsRef<Path>, P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>
        // given paths to all of the lists of stuff, create a new Template_manager
    {
        Template_manager {
            templates: Phrase::load_from_file(template_path),
            nouns: Phrase::load_from_file(noun_path),
            adjectives: Phrase::load_from_file(adj_path),
            adverbs: Phrase::load_from_file(adv_path),
        }
    }

    fn get_noun_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        // we juggle Vec<u8>s around here because they're easy to iterate through and replace bytes
        // this method and its friends get_adjective_phrase and get_adverb_phrase choose a random phrase from the Vec<Phrase>s
        if let Some(ref noun_phrase) = rng.choose(&self.nouns) {
            noun_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get a noun phrase")
        }
    }

    fn get_adjective_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        if let Some(ref adj_phrase) = rng.choose(&self.adjectives) {
            adj_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get an adjective phrase")
        }
    }

    fn get_adverb_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        if let Some(ref adv_phrase) = rng.choose(&self.adverbs) {
            adv_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get an adjective phrase")
        }
    }

    fn format_phrase(&self, phrase: Vec<u8>, mut rng: &mut ThreadRng) -> Vec<u8> {
        // this method iterates through a Vec<u8> and replaces any of the FORMAT_CHARs with a phrase
        // it's recursive so that adjectives can be things like "%-driven"
        let mut new_bytes: Vec<u8> = Vec::new();
        for byte in phrase.iter() {
            if *byte == NOUN_FORMAT_CHAR {
                for noun_byte in self.format_phrase(self.get_noun_phrase(&mut rng), &mut rng).iter() {
                    new_bytes.push(*noun_byte);
                }
            } else if *byte == ADJECTIVE_FORMAT_CHAR {
                for adj_byte in self.format_phrase(self.get_adjective_phrase(&mut rng), &mut rng).iter() {
                    new_bytes.push(*adj_byte);
                }
            } else if *byte == ADVERB_FORMAT_CHAR {
                for adv_byte in self.format_phrase(self.get_adjective_phrase(&mut rng), &mut rng).iter() {
                    new_bytes.push(*adv_byte);
                }
            } else {
                new_bytes.push(*byte);
            }
        }
        new_bytes
    }

    pub fn make_formatted_quote(&self, mut rng: &mut ThreadRng) -> String {
        // this is the big wrapper method that main.rs/main() calls
        // it returns a fully formatted phrase that can be tweeted
        // note: I can't figure out how to have the Template_handler own rng
        // because getting a random value mutates the rng
        // so instead it's owned by main() and these methods just pass refs to it back and forth
        if let Some(ref template) = rng.choose(&self.templates) {
            String::from_utf8(self.format_phrase(template.text.clone().into_bytes(), &mut rng)).unwrap()
        } else {
            panic!("couldn't get a template")
        }
    }
}
