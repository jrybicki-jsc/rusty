fn main() {
    let favorite_color: Option<&str> = Some("red");

    if let Some(color) = favorite_color {
        println!("Using favorite color {}", color);
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} at index {}", value, index);
    }
}
