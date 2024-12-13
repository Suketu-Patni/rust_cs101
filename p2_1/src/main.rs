use std::io;

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        let mut harmonic_sum = 0f64;
        for i in 1..(ni+1) {
            harmonic_sum += (1f64)/(i as f64);
        }
        println!("{:.10}", harmonic_sum);

    }
}