use itertools::Itertools;
use std::fs;
use std::io;

fn is_pal(word: String) -> bool {
    let rev = word.chars().rev().collect::<String>();

    rev == word
}

fn read_dic() -> String {
    let filename = String::from("./dictionary.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");

    contents
}

fn find_pals(contents: String) {
    for line in contents.lines() {
        if is_pal(line.to_string()) {
            println!("Found: {}", line);
        }
    }
}

fn is_anag(word: String, line: String) -> bool {
    let ww = word.chars().sorted().collect::<String>();
    let other = line.chars().sorted().collect::<String>();

    other == ww
}

fn find_anag(word: String, dict: String) {
    let ww = word.chars().sorted().collect::<String>();

    println!("Checking for: {}", ww);

    for line in dict.lines() {
        if line.len() != word.len() {
            continue;
        }
        let other = line.chars().sorted().collect::<String>();
        if ww.eq_ignore_ascii_case(&other) {
            println!("Found anagram: {}", line);
        }
    }
}

fn main() {
    let contents = read_dic();
    // find_pals(contents);
    println!("Enter the word:");
    let mut g = String::new();
    io::stdin()
        .read_line(&mut g)
        .expect("Failed to read string");
    let g = g.replace("\n", "");

    find_anag(g, contents);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert!(!is_pal(String::from("Bla")));
        assert!(is_pal(String::from("kayak")));
    }

    #[test]
    fn anag() {
        assert!(is_anag(String::from("post"), String::from("stop")));
        assert!(!is_anag(String::from("bla"), String::from("ola")));
    }
}
