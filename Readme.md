# Anagram

Find anagrams for words

## Usage

#### build it
```shell
cargo build --release
```
#### run it
```shell
target/release/anagram <word> [<word> ...]
```

#### example
```shell
sh-3.2$ target/release/anagram Aníbal Algés
[Aníbal]
 - albina
[Algés]
 - gales
 - galés
 - galês
 - gelas
 - legas
 ```

## Notes
- The wordlist file name is hardcoded. Its path is `assets/wordlist.txt`. Didn't bother to make this as a command line 
argument;
- Only the portuguese wordlist is supplied. To find anagrams for words in other languages, you need to supply your own
wordlist file;  
- The wordlist was taken from here: https://natura.di.uminho.pt/download/sources/Dictionaries/wordlists;
- Anagrams are searched using lowercased normalized words. For example, `Aníbal` will match `albina` and vice-versa;
