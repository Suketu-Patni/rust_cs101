use std::io;

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        let mut r_n = ni as f64;
        let mut outside_multiplier = ni - 1;

        loop {
            r_n = (outside_multiplier as f64) * f64::powf(r_n + 1f64, 0.5);
            outside_multiplier -= 1;
            if outside_multiplier == 0 {
                break;
            }
        }

        println!("{:.10}", r_n);

    }
}