use std::io;

fn harmonic_mean(n: f64, xi_vector: &Vec<f64>) -> f64 {
    let mut reciprocal_sum = 0f64;
    for xi in xi_vector {
        reciprocal_sum += (1 as f64)/xi;
    }
    return n/reciprocal_sum;
}

fn geometric_mean(n: f64, xi_vector: &Vec<f64>) -> f64 {
    let mut gm = 1f64;
    for xi in xi_vector {
        gm *= f64::powf(*xi, (1 as f64)/n);
    }
    return gm;
}

fn rms(n: f64, xi_vector: &Vec<f64>) -> f64 {
    return f64::powf(xi_vector.iter().map(|x| x*x).sum::<f64>()/n, 0.5);
}

fn cbrtmcube(n: f64, xi_vector: &Vec<f64>) -> f64 {
    return f64::powf(xi_vector.iter().map(|x| x*x*x).sum::<f64>()/n, 1.0/3.0);
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim().parse::<usize>().unwrap();

    let mut mp_vector:Vec<(f64, f64, f64, f64, f64, f64, f64)> = vec![];

    for _ in 0..t {
        let mut xni_string = String::new();
        io::stdin().read_line(&mut xni_string).expect("failed to read string of integers");
        let mut xni_vector:Vec<f64> = xni_string.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
        xni_vector.reverse();
        let ni = xni_vector.pop().unwrap();
        xni_vector.reverse();

        let min = xni_vector.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = xni_vector.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let summed:f64 = xni_vector.iter().sum();
        let mean = summed/ni;
        mp_vector.push((*min, 
            harmonic_mean(ni, &xni_vector), 
            geometric_mean(ni, &xni_vector), 
            mean,
            rms(ni, &xni_vector),
            cbrtmcube(ni, &xni_vector),
            *max
        ));

    }

    for means in mp_vector {
        // print!("{} {} {} {} {}", mp_vector.)
        println!("{:.5} {:.5} {:.5} {:.5} {:.5} {:.5} {:.5}", means.0, means.1, means.2, means.3, means.4, means.5, means.6);
    }
}