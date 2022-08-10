fn main() {
    let r = 5;

    let r = r + 1;

    {
        let r = r * 2;
        println!("The value of r in the inner scope is: {r}");
    }

    println!("The value of r is: {r}");

    // Tuple Structs
    let tup = (500, 6.4, 1);

    //Destructuring the tuple
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    //let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    test_func();
}

fn test_func() {
    println!("Another function")
}
