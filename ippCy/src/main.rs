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

fn encode(plaintext: String) -> String {
    //capitalize and remove spac
    let plaintext: String = plaintext.to_uppercase().split_whitespace().collect();

    let a1 = plaintext.chars().enumerate().filter(|(a, b)| a%2 == 0).map(|(a,b)| b).collect::<String>();
    let a2 = plaintext.chars().enumerate().filter(|(a, b)| a%2 == 1).map(|(a,b)| b).collect::<String>();

    format!("{}{}", a1, a2)
}

fn decode(mut enc: String) -> String {
    let lower = enc.split_off(enc.len() / 2);

    let myi = enc.chars().zip(lower.chars());
    let mut ret = String::new();

    for (a, b) in myi {
        ret.push(a);
        ret.push(b);
    }

    ret
}

fn main() {
    //railfence:
    let plaintext =
        String::from("Let us cross over the river and rest under the shade of the trees");

    let mut enc = encode(plaintext);
    println!("Encode\n{}", enc);

    let dec = decode(enc);
    println!("Decode\n{}", dec);

}

fn get_ij(i: usize, j: usize, matrix: &Vec<u8>) -> u8 {
    let cols = 4;
    let rows = 5;

    matrix[i + j * rows]
}
