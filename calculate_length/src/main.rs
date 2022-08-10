fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("Hello");
    let s3 = "Hello world";
    // Example of scopes
    {
        let r1 = &mut s2;
    } //? r1 goes out of scope so we can create new reference

    let r2 = &mut s2;

    //Calling functions
    change(&mut s2);

    // borrowing s1
    let len = calculate_length(&s1);

    println!("The length of {} is {}, {}", s1, len, s2);

    //Example of slices
    // We are borrowing s2
    let word = first_world(s3);

    println!("The first word is {}", word);
}

// local variable s is a reference to s2
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope and is dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//* Example of slices
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
