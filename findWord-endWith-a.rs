use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("src/text.txt").unwrap();
    let reader = BufReader::new(file);
    let mut max_word = String::new();
    let mut max_len = 0;
    let mut cntr = 0;
    // считать строки
    for line in reader.lines() {
        // найти самое длинное слово заканчивающееся на "а" и его id в массивe
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        while let Some(word) = words.next() {
            if word.ends_with("а") {
                if word.len() > max_len && word.len() > cntr{
                    max_len = word.len();
                    max_word = word.to_string();
                    cntr = word.len();
                }
            }
        }  
    }
    println!("Самое длинное слово заканчивающееся на \"а\" - {}", max_word);
    println!("Номер слова с самой длинной а - {}", cntr);
}   
