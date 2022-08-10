fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    expression();

    let x = plus_one(2);

    println!("Value of x: {}", x);
}

fn another_function(x: i32, label: char) {
    println!("Another function x: {x}, {label}");
}

fn expression() {
    let y = {
        let x = 3;
        //Expression does not include ending semicolon!
        x + 1
    };

    println!("The value of y is: {y}");
}

//Return func
fn five() -> i32 {
    //Expression
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
