extern crate rand;

use rand::{thread_rng, ThreadRng};

const NOUNS_FILENAME: &str = "resources/nouns.txt";
const TEMPLATE_FILENAME: &str = "resources/templates.txt";
const ADJECTIVES_FILENAME: &str = "resources/adjectives.txt";
const ADVERBS_FILENAME: &str = "resources/adverbs.txt";
const ABSTRACTS_FILENAME: &str = "resources/abstracts.txt";

mod template_manager;
use template_manager::TemplateManager;

mod tweet_manager;
use tweet_manager::new_tweet;

fn main() {
    match TemplateManager::new(
        TEMPLATE_FILENAME,
        NOUNS_FILENAME,
        ADJECTIVES_FILENAME,
        ADVERBS_FILENAME,
        ABSTRACTS_FILENAME
    ) {
        Ok(manager) => {
            let mut rng: ThreadRng = thread_rng();
            match manager.make_formatted_quote(&mut rng) {
                Some(quote) => {
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
