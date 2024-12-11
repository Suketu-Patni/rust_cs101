use std::io;

fn main() {
    let mut _t = String::new();
    io::stdin().read_line(&mut _t).expect("Failed to read line");

    let mut ni_string = String::new();
    io::stdin().read_line(&mut ni_string).expect("failed to read string of integers");

    let ni_vector:Vec<u32> = ni_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    for ni in ni_vector {
        if ni == 0 {
            println!("1");
        } else if ni == 1 {
            println!("1 2");
        } else {
            println!("1\n1 2");
            let mut numbers_vec: Vec<u32> = vec![1,2];
            let mut current_vector_length = 2;
            for _ in 0..ni-1 {
                let mut new_vec:Vec<u32> = vec![0; current_vector_length + 1];
                new_vec[0] = 1;
                for i in 1..current_vector_length {
                    new_vec[i] = numbers_vec[i-1] + numbers_vec[i];
                }
                new_vec[current_vector_length] = 2 * numbers_vec[current_vector_length - 1];
                for element in &new_vec {
                    print!("{} ", element);
                }
                println!("");
                numbers_vec = new_vec;
                current_vector_length = current_vector_length + 1;
            }
        }
        println!("");
    }
}