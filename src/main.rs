use std::io::{stdin, stdout, Write};

use spell_checker_bloom_filters::{
    bloom_filters::BloomFilter,
    spell_checker::{LocalStorage, SpellChecker, StorageService},
    weak_password_detector::{DetectError, PasswordDetector},
};

fn main() {
    use_password_detector();

    use_spell_checker();
}

fn use_password_detector() {
    let (buffer, storage) = init_buffer_and_storage();

    let detector = PasswordDetector::builder()
        .with_buffer(buffer)
        .with_database(storage)
        .build()
        .unwrap();

    let mut input = String::new();

    print!("Write a password: ");

    stdout().flush().unwrap();

    stdin().read_line(&mut input).expect("Failed to read line");

    let words = input.split_whitespace();

    for word in words {
        match detector.verify(word) {
            DetectError::Initialize(value) => println!("{}", value),
            DetectError::Storage(err) => println!("{:?}", err),
            DetectError::Dismiss => {
                println!("{} is a common password. Please try with another one", word)
            }
            DetectError::Approve => println!("Your password is not common"),
        }
    }
}

fn use_spell_checker() {
    let (buffer, storage) = init_buffer_and_storage();

    let spelling_checker = SpellChecker::builder()
        .with_buffer(buffer)
        .with_database(storage)
        .build()
        .unwrap();

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

fn init_buffer_and_storage() -> (BloomFilter, Box<dyn StorageService>) {
    let buffer = BloomFilter::builder().build().unwrap();

    let storage = Box::new(
        LocalStorage::builder()
            .with_storage_location("database.txt")
            .build()
            .unwrap(),
    );

    (buffer, storage)
}
