use std::fs;

fn is_pal(word: String) -> bool {
    let rev = word.chars().rev().collect::<String>();

    rev == word
}

fn main() {
    let filename = String::from("./dictionary.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");

    is_pal(String::from("Hello"));

    for line in contents.lines() {
        if is_pal(line.to_string()) {
            println!("Found: {}", line);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert!(!is_pal(String::from("Bla")));
        assert!(is_pal(String::from("kayak")));
    }
}
