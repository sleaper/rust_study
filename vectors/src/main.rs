fn main() {
    //New vector
    let _v: Vec<i32> = Vec::new();

    //Vector with values
    let v2 = vec![1, 2, 3, 4, 5, 6];

    //Adding elements
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);

    /*
    Reading elements
    */
    let third: &i32 = &v2[2];
    println!("The third element in vector {}", third);

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // get return NONE
    // normal acces panic the program
    //
    // let _does_not_exists: &i32 = &v2[100];
    let _does_not_exists: Option<&i32> = v2.get(100);

    //For loop e.g.
    for i in &mut v3 {
        *i += 50;
    }

    //Enum with vector

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} //<-- vectors go out of scope and are freed here
