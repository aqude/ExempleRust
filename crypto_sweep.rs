use std::char::decode_utf16;
use std::fs;
use std::path::Path;

fn main() {
    // read file
    let path = Path::new("/home/aqude/CLionProjects/crypto/src/text.txt");
    let str = fs::read_to_string(path).expect("Error in reading the file");
    let crt = crypto(str);
    println!("{}", crt);
}

fn crypto(text: String) -> String {
    let mut result = String::new();
    let mut v: Vec<u16> = text.encode_utf16().collect();
    for i in 0..v.len() {
        v[i] = v[i] + 3;
    }
    let iter = v.iter().map(|&x| x);
    let mut iter2 = decode_utf16(iter);
    while let Some(c) = iter2.next() {
        result.push(c.unwrap());
    }
    result
}

fn decode(text: String) -> String {
    let mut result = String::new();
    let mut v: Vec<u16> = text.encode_utf16().collect();
    for i in 0..v.len() {
        v[i] = v[i] - 3;
    }
    let iter = v.iter().map(|&x| x);
    let mut iter2 = decode_utf16(iter);
    while let Some(c) = iter2.next() {
        result.push(c.unwrap());
    }
    result
}
