use std::io;

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        let mut c1_n = ni as f64;
        let mut term = ni;
        let mut c2_n = 1f64;

        let mut c2_n_denom = 1f64;

        for i in 0..ni {
            c2_n_denom *= (2 * i + 3) as f64;
            c2_n += (1 as f64)/c2_n_denom;
        }

        let mut stop_flag = 0;

        loop {
            if (term == 0) | (stop_flag == 1) {
                break;
            } 
            c1_n += 1 as f64;           
            if term != 1 {
                term -= 1;
            } else {
                stop_flag = 1;
            }
            c1_n = (term as f64)/c1_n;            
        }

        // if ni != 0 {
        //     c1_n = (1 as f64)/c1_n;
        // }
        println!("{:.10}", c1_n + c2_n);

    }
}