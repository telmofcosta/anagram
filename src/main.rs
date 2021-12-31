use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_words(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn is_anagram(entry: &String, word: &String) -> bool {
    if entry.len() != word.len() { return false };

    true
}

fn main() {
    let dict = read_words("assets/words.txt");
    let word = "telmo".to_string();

    let anagrams: Vec<&String> = dict.iter().filter(|dict_entry| is_anagram(dict_entry, &word)).collect();

    println!("{:?}", anagrams);
}
