use std::io;
// use std::cmp::Ordering;
fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        for i in 1..(2 * ni + 2) {
            let mut final_string = String::new();
            if i == ni + 1 {
                for _ in 1..(2*ni+2) {
                    final_string.push_str("*");
                }
                println!("{}", final_string);
            } else {
            
                let num_stars_on_each_side: u32 = ni + 1 - (ni + 1).abs_diff(i);
                for _ in 0..num_stars_on_each_side {
                    final_string.push_str("*");
                }
                let num_spaces_in_middle: u32 = 2 * ni + 1 - 2 * num_stars_on_each_side;
                for _ in 0..num_spaces_in_middle {
                    final_string.push_str(" ");
                }
                for _ in 0..num_stars_on_each_side {
                    final_string.push_str("*");
                }
                println!("{}", final_string);
            }
        }
        println!("");
    }
}
