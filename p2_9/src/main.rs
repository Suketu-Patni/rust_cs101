use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim().parse::<usize>().unwrap();

    let mut area_vector:Vec<f64> = vec![];

    for _ in 0..t {
        let mut xni_yni_string = String::new();
        io::stdin().read_line(&mut xni_yni_string).expect("failed to read string of integers");
        let xni_yni_vector:Vec<i32> = xni_yni_string.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let ni = xni_yni_vector[0] as usize;
        let mut xi_vector:Vec<i32> = vec![];
        let mut yi_vector:Vec<i32> = vec![];

        for i in 1..(2 * ni + 1) {
            if i % 2 != 0 {
                xi_vector.push(xni_yni_vector[i]);
            } else {
                yi_vector.push(xni_yni_vector[i]);
            }
        }
        
        let mut area = 0i32;

        for j in 0..(ni - 1) {
            area += xi_vector[j] * yi_vector[j + 1] - xi_vector[j + 1] * yi_vector[j];
        }

        area += xi_vector[ni - 1] * yi_vector[0] - xi_vector[0] * yi_vector[ni - 1];
        area_vector.push((area as f64)/2.0);
    }

    for area in area_vector {
        println!("{:.1}", area);
    }
}