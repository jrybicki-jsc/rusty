use std::fs;

fn reveal(contents: &str, lookahead: u8) -> String {
    let mut counter = 0;

    let mut ret = String::new();

    for c in contents.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }

        counter += 1;

        if c.is_ascii_punctuation() {
            counter = 0;
        }
        if counter == lookahead {
            print!("{}", c.to_uppercase());
            ret.push(c);
        } else {
            print!("{}", c);
        }
    }

    ret
}

fn main() {
    let filename = String::from("./message.txt");
    let contents = fs::read_to_string(filename).expect("Failed to open dictionary");

    let m = reveal(&contents, 3);
    println!("\n{}", m);
}
