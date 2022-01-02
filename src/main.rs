extern crate unidecode;
use unidecode::unidecode;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::process::exit;

#[derive(Debug)]
struct Entry {
    word: String,
    normalized_word: String,
    bytes: Vec<u8>,
}

fn read_words(filename: impl AsRef<Path>) -> Vec<Entry> {
    let file = File::open(filename).expect("no such file");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|line| word_to_entry(line))
        .collect()
}

fn word_to_entry(word: String) -> Entry {
    let normalized_word = unidecode(word.clone().to_lowercase().as_str());
    let mut word_vec = normalized_word.as_bytes().to_vec();
    word_vec.sort_unstable();

    Entry {
        word,
        normalized_word,
        bytes: word_vec,
    }
}

fn is_anagram(entry: &Entry, target: &Entry) -> bool {
    if entry.normalized_word.len() != target.normalized_word.len() { return false };

    if target.normalized_word == entry.normalized_word { return false };
    if entry.bytes == target.bytes { return true }

    return false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Specify at least one word"); exit(1)
    }

    // let dict = read_words("assets/words.txt");
    let dict = read_words("assets/wordlist-big-20211125.txt");

    for i in 1..args.len() {
        let word_entry = word_to_entry(args[i].clone());
        let anagrams: Vec<&Entry> = dict.iter().filter(|dict_entry| is_anagram(dict_entry, &word_entry)).collect();
        println!("[{}]", args[i]);
        for entry in anagrams {
            println!(" - {}", entry.word);
        }
    }
}
