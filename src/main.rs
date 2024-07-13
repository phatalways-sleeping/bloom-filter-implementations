use std::io::{stdin, stdout, Write};

use spell_checker_bloom_filters::{
    bloom_filters::BloomFilter,
    spell_checker::{LocalStorage, SpellChecker},
};

fn main() {
    let spelling_checker = init_spelling_checker();

    let mut input = String::new();

    print!("Write something: ");
    
    stdout().flush().unwrap();
    
    stdin().read_line(&mut input).expect("Failed to read line");

    let words = input.split_whitespace();

    for word in words {
        if !spelling_checker.check_spelling_of(word).unwrap() {
            println!("{} may have been wrongly typed", word);
        }
    }
}

fn init_spelling_checker() -> SpellChecker {
    let buffer = BloomFilter::builder().build().unwrap();

    let database = LocalStorage::builder()
        .with_storage_location("database.txt")
        .build()
        .unwrap();

    SpellChecker::builder()
        .with_buffer(buffer)
        .with_database(Box::new(database))
        .build()
        .unwrap()
}
