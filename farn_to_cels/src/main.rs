use std::io;

fn main() {
    println!("Hello, world!");

    println!("Chose your temperature type:");
    println!("1) Celsius");
    println!("2) Fahrenheit");

    let mut user_tmp = String::new();

    io::stdin()
        .read_line(&mut user_tmp)
        .expect("Failed to read line");

    let user_tmp: i32 = match user_tmp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };

    let tmp_type: i32 = match user_tmp {
        1 => 1,
        2 => 2,
        _ => panic!("Not a valid option"),
    };

    println!("Write temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };

    let converted = calculate_tmp(tmp_type, temperature);

    if tmp_type == 1 {
        println!("Temperature is {converted}F");
    } else {
        println!("Temperature is {converted}C");
    }
}

fn calculate_tmp(tmp_type: i32, temperature: i32) -> f64 {
    if tmp_type == 2 {
        //Celsius to Fahrenheit
        ((f64::from(temperature) - 32.0) * (5.0 / 9.0)).floor()
    } else {
        //Fahrenheit to celsius
        (f64::from(temperature) * (9.0 / 5.0) + 32.0).floor()
    }
}
