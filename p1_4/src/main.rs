use std::io;
// use std::cmp::Ordering;
fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        let mut letter = 'A';
        let mut letter_ascii = 65u8;
            for i in 1..(ni + 1) {
                for _ in 0..i {
                    print!("{}", letter);
                    if letter == 'Z' {
                        letter = 'A';
                        letter_ascii = 65;
                    } else{
                        letter_ascii = letter_ascii + 1;
                        letter = letter_ascii as char;
                    }
                }
                println!("");
            }
        println!("");
    }
}