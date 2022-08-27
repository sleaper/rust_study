fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut count = 0;
    while count < 10 {
        println!("count = {count}");
        count += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for el in a {
        println!("x = ");
    }
}
