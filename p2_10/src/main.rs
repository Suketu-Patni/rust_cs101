use std::io;

fn f(x: f64) -> f64 {
    return f64::powf(x, 4.0) * f64::powf(1.0 - x, 4.0) / (1.0 + x * x);
}

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        // n ordinates -> n-1 subintervals
        let delta_x = 1.0/((ni as f64) - 1.0);
        let mut simpson_sum = f(0.0) + f(1.0);

        // below is from wikipedia. i didn't understand alternation
        for i in 1..((ni - 1)/2 + 1) {
            simpson_sum += f(((2*i-2) as f64) * delta_x) + 4.0 * f(((2*i-1) as f64) * delta_x) + f(((2*i) as f64) * delta_x);
        }

        simpson_sum *= delta_x/3.0;


        println!("{:.15}", 22.0/7.0 - simpson_sum);

    }
}