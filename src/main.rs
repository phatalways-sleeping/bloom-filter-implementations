use spell_checker_bloom_filters::{
    bloom_filters::BloomFilter,
    spell_checker::{LocalStorage, SpellChecker},
};

fn main() {
    let buffer = BloomFilter::from(10_000_000, Some(0.01), None).unwrap();

    let database = LocalStorage::try_from("database.txt").unwrap();

    let spell_checker = SpellChecker::builder()
        .with_buffer(buffer)
        .with_database(Box::new(database))
        .build()
        .unwrap();

    let words = vec!["hot", "cold", "helo", "catastrophic", "coding", "challenges", "imadethis"];

    for word in words {
        let result = spell_checker.check_spelling_of(word).unwrap();
        if !result {
            println!("Word {} has spelling errors", word);
        }
    }
}
