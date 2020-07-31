fn route_ci() {
    let ciphertext = "16 12 8 4 0 1 5 9 13 17 18 14 10 6 2 3 7 11 15 19";
    let cols = 4;
    let rows = 5;

    let mut sp = ciphertext.split(" ").collect::<Vec<&str>>();
    let mut matrix: Vec<u8> = Vec::new();

    let keys = "-1 2 -3 4";
    for key in keys.split(" ") {
        let k: i8 = key.parse().unwrap();

        let a = if k < 0 {
            sp.drain(0..rows).collect::<Vec<&str>>()
        } else {
            sp.drain(0..rows).rev().collect::<Vec<&str>>()
        };
        for el in a {
            print!("{}\t", el);
            matrix.push(el.parse().unwrap());
        }
        println!("");
    }

    //decode matrix:
    println!("Decoded message is:");
    for a in (0..rows).rev() {
        for b in 0..cols {
            //println!("m[{}, {}]={}", a, b, get_ij(a, b, &matrix));
            print!("{} ", get_ij(a, b, &matrix));
        }
    }
}

fn main() {
    //railfence:
    let plaintext =
        String::from("Let us cross over the river and rest under the shade of the trees");
    //capitalize and remove spac
    let plaintext: String = plaintext.to_uppercase().split_whitespace().collect();

    let mut upper: Vec<char> = Vec::new();
    let mut lower: Vec<char> = Vec::new();

    for (p, o) in plaintext.chars().enumerate() {
        if p % 2 == 0 {
            upper.push(o);
        } else {
            lower.push(o);
        }
        if (p > 0) && (p % 8 == 0) {
            upper.push(' ');
            lower.push(' ');
        }
    }

    let mut upper = upper.iter().collect::<String>();
    let mut lower = lower.iter().collect::<String>();

    print!("{}\n{}", upper, lower);
}

fn get_ij(i: usize, j: usize, matrix: &Vec<u8>) -> u8 {
    let cols = 4;
    let rows = 5;

    matrix[i + j * rows]
}
