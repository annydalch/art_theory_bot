extern crate rand;

use std::path::Path;
use rand::{Rng, ThreadRng};

const NOUN_FORMAT_CHAR: u8 = b'%';
const ADJECTIVE_FORMAT_CHAR: u8 = b'#';
const ADVERB_FORMAT_CHAR: u8 = b'@';
const ABSTRACT_FORMAT_CHAR: u8 = b'$';

mod phrase;
use self::phrase::Phrase;
// Phrase is just a wrapper around a String with a method to load a Vec of them from a file

pub struct TemplateManager {
    // this is a struct that handles all of the actual text generation
    // we use Vec<Phrase> so we can use Rng.choose()
    templates: Vec<Phrase>,
    nouns: Vec<Phrase>,
    adjectives: Vec<Phrase>,
    adverbs: Vec<Phrase>,
    abstracts: Vec<Phrase>,
}


impl TemplateManager {
    pub fn new<P0, P1, P2, P3, P4>(template_path: P0, noun_path: P1, adj_path: P2, adv_path: P3, abstract_path: P4) -> TemplateManager
        where P0: AsRef<Path>, P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>, P4: AsRef<Path>
    // given paths to all of the lists of stuff, create a new Template_manager
    {
        TemplateManager {
            templates: Phrase::load_from_file(template_path),
            nouns: Phrase::load_from_file(noun_path),
            adjectives: Phrase::load_from_file(adj_path),
            adverbs: Phrase::load_from_file(adv_path),
            abstracts: Phrase::load_from_file(abstract_path),
        }
    }

    fn get_noun_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        // we juggle Vec<u8>s around here because they're easy to iterate through and replace bytes
        // this method and its friends get_adjective_phrase and get_adverb_phrase choose a random phrase from the Vec<Phrase>s
        if let Some(noun_phrase) = rng.choose(&self.nouns) {
            noun_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get a noun phrase")
        }
    }

    fn get_adjective_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        if let Some(adj_phrase) = rng.choose(&self.adjectives) {
            adj_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get an adjective phrase")
        }
    }

    fn get_adverb_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        if let Some(adv_phrase) = rng.choose(&self.adverbs) {
            adv_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get an adjective phrase")
        }
    }

    fn get_abstract_phrase(&self, mut rng: &mut ThreadRng) -> Vec<u8> {
        if let Some(abstract_phrase) = rng.choose(&self.abstracts) {
            abstract_phrase.text.clone().into_bytes()
        } else {
            panic!("couldn't get an abstract phrase")
        }
    }

    fn format_phrase(&self, phrase: Vec<u8>, mut rng: &mut ThreadRng) -> Vec<u8> {
        // this method iterates through a Vec<u8> and replaces any of the FORMAT_CHARs with a phrase
        // it's recursive so that adjectives can be things like "%-driven"
        let mut new_bytes: Vec<u8> = Vec::new();
        for byte in &phrase {
            match *byte {
                NOUN_FORMAT_CHAR => {
                    for noun_byte in &self.format_phrase(self.get_noun_phrase(&mut rng), &mut rng) {
                        new_bytes.push(*noun_byte);
                    }
                },
                ADJECTIVE_FORMAT_CHAR => {
                    for adj_byte in &self.format_phrase(self.get_adjective_phrase(&mut rng), &mut rng) {
                        new_bytes.push(*adj_byte);
                    }
                },
                ADVERB_FORMAT_CHAR => {
                    for adv_byte in &self.format_phrase(self.get_adverb_phrase(&mut rng), &mut rng) {
                        new_bytes.push(*adv_byte);
                    }
                },
                ABSTRACT_FORMAT_CHAR => {
                    for abstract_byte in &self.format_phrase(self.get_abstract_phrase(&mut rng), &mut rng) {
                        new_bytes.push(*abstract_byte);
                    }
                },
                _ => {
                    new_bytes.push(*byte);
                }
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
        if let Some(template) = rng.choose(&self.templates) {
            String::from_utf8(self.format_phrase(template.text.clone().into_bytes(), &mut rng)).unwrap()
        } else {
            panic!("couldn't get a template")
        }
    }
}
