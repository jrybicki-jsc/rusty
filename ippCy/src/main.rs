fn main() {
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

fn get_ij(i: usize, j: usize, matrix: &Vec<u8>) -> u8 {
    let cols = 4;
    let rows = 5;

    matrix[i + j * rows]
}
