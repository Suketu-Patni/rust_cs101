use std::io;

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        let mut two_by_pi_ni = 1f64;
        let mut c_j = 0f64;

        for _ in 0..ni {
            c_j += 2 as f64;
            c_j = f64::powf(c_j, 0.5);
            two_by_pi_ni *= c_j/(2 as f64);
        }

        println!("{:.15}", (2 as f64)/two_by_pi_ni);

    }
}