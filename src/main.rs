extern crate rand;

use rand::{thread_rng, ThreadRng};

const NOUNS_FILENAME: &str = "resources/nouns.txt";
const TEMPLATE_FILENAME: &str = "resources/templates.txt";
const ADJECTIVES_FILENAME: &str = "resources/adjectives.txt";
const ADVERBS_FILENAME: &str = "resources/adverbs.txt";
const ABSTRACTS_FILENAME: &str = "resources/abstracts.txt";

mod template_manager;
use template_manager::TemplateManager;

fn main() {
    let manager: TemplateManager = TemplateManager::new(
        TEMPLATE_FILENAME,
        NOUNS_FILENAME,
        ADJECTIVES_FILENAME,
        ADVERBS_FILENAME,
        ABSTRACTS_FILENAME
    );
    let mut rng: ThreadRng = thread_rng();
    println!("{}", manager.make_formatted_quote(&mut rng));
}
