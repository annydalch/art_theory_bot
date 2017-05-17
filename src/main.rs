extern crate rand;

use rand::{thread_rng, ThreadRng};

// these files are paths to plain-text lists of the various word types
// these paths assume you're running with *cargo run* from the root of the crate
// for production, i replace these with absolute paths
const NOUNS_FILENAME: &str = "resources/nouns.txt";
const TEMPLATE_FILENAME: &str = "resources/templates.txt";
const ADJECTIVES_FILENAME: &str = "resources/adjectives.txt";
const ADVERBS_FILENAME: &str = "resources/adverbs.txt";
const ABSTRACTS_FILENAME: &str = "resources/abstracts.txt";

// this module does the heavy lifting of generating the text for tweets
// because i'm lazy, we do templating and replacement instead of markov chaining
mod template_manager;
use template_manager::TemplateManager;

// this module is very light
// the function new_tweet sends a tweet given an AsRef<str> of text to tweet
mod tweet_manager;
use tweet_manager::new_tweet;

fn main() {
    // TemplateManager::new returns a Result because it will fail if there's a problem loading any of the files
    match TemplateManager::new(
        TEMPLATE_FILENAME,
        NOUNS_FILENAME,
        ADJECTIVES_FILENAME,
        ADVERBS_FILENAME,
        ABSTRACTS_FILENAME
    ) {
        Ok(manager) => {

            // i feel like there's probably a way to have the TemplateManager own ThreadRng
            // but i'm not sure what it is - mutability in structs is hard
            let mut rng: ThreadRng = thread_rng();

            // TemplateManager.make_formatted_quote returns an Option
            // because it doesn't work if any of the lists of phrases are empty
            match manager.make_formatted_quote(&mut rng) {
                Some(quote) => {
                    // new_tweet returns an empty Result because the networking can fail
                    if let Err(err) = new_tweet(&quote) {
                        println!("Failed to tweet with err: {}", err.description());
                        println!("The tweet was '{}'", quote);
                    } else {
                        println!("Tweeted '{}'", quote)
                    }
                },
                None => {
                    println!("Couldn't generate a tweet");
                },
            }
        },
        Err(err) => {
            println!("Failed to create a TemplateManager with err: {}", err);
        },
    }
}
