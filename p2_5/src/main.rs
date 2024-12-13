use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim().parse::<usize>().unwrap();

    let mut pq_vector:Vec<(i64, i64)> = vec![];

    for _ in 0..t {
        let mut ani_string = String::new();
        io::stdin().read_line(&mut ani_string).expect("failed to read string of integers");
        let ani_vector:Vec<i64> = ani_string.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        let mut p = 1i64;
        let mut q = ani_vector[1] as i64;

        for i in 2..(ani_vector[0] + 2) {
            let ani = ani_vector[i as usize];
            let temp = p;
            p = q; // p = a_n
            q = p * ani + temp;// q = a_n * a_(n-1) + 1
        }

        pq_vector.push((p, q));

    }

    for pq in pq_vector {
        println!("{}/{}", pq.1, pq.0);
    }
}