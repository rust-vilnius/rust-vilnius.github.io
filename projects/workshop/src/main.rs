extern crate reqwest;
extern crate serde_json;
extern crate ammonia;

use ammonia::Ammonia;
use std::collections::HashSet;

pub mod wiki;

fn main() {
    let page = wiki::search("rust").expect("failed to find page");
    let html = page.get_contents();

    let defaults = Ammonia::default();
    let ammonia = ammonia::Ammonia { tags: HashSet::new(), ..defaults };

    let clean_text = ammonia.clean(&html);

    // split by whitespace and collect into hash set of string references
    let unique_words: HashSet<&str> = clean_text
        .split_whitespace()
        .collect();

    println!("Unique word count in {:?}: {:?}", page.get_title(), unique_words.len());
}