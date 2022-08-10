use std::io;

fn main() {
    println!("Writhe nth number you want");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };

    let seq = fibonacci(num);

    println!("The fibonacci {num}th is: {seq}");
}

fn fibonacci(n: i64) -> i64 {
    println!("number {n}");
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
//4
// fb 3 + fb 2
// fb 2 + fb 1 // fb 1 + fb 0
// fb 1 + fb 0 // 1 // 1 // 0
// 1 // 0 // 1 // 1 // 0
