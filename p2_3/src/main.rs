use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim().parse::<usize>().unwrap();

    let mut ai_ni_string = String::new();
    io::stdin().read_line(&mut ai_ni_string).expect("failed to read string of integers");

    let ai_ni_vector:Vec<f64> = ai_ni_string.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();

    for i in 0..t {
        let ai = ai_ni_vector[2*i];
        let ni = ai_ni_vector[2*i+1] as u32;
        let mut tetration = ai;

        for _ in 0..ni-1 {
            tetration = f64::powf(ai, tetration);
        }

        println!("{:.10}", tetration);

    }
}