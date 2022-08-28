use rand::Rng;
use std::io;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut size = String::new();
    println!("Type a size of vector: ");

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: i32 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };

    for _i in 0..size {
        let rnd_number: i32 = rand::thread_rng().gen_range(1..=100);
        vec.push(rnd_number);
    }

    println!("Median of vector: {}", median(&vec));
}

fn median(vec: &Vec<i32>) -> i32 {
    let mut vec = vec.clone();
    vec.sort();
    if vec.len() % 2 == 0 {
        let middle = vec.len() / 2;
        let median = (vec[middle - 1] + vec[middle]) / 2;
        return median;
    } else {
        let middle = vec.len() / 2;
        return vec[middle];
    }
}
