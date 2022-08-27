fn main() {
    //String is just a wrapper around a Vec<T>

    let mut s = String::new();

    //string slice
    let data = "initial contents";

    //Are the same
    let mut s2 = String::from(data);
    let s = data.to_string();

    //Appanding to a string
    s2.push_str("foo");
    println!("s2: {}", s2);

    let s3 = String::from("foo");
    //s2 will not be valid, add take s2 ownership
    let _s3 = s2 + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    iterate_string();
}

fn string_indexing() {
    //In rust we cant index strings, we have to use string slices
    let s = String::from("hello");
    let h = &s[0..1];
}

fn iterate_string() {
    let s = String::from("hello world");

    //iterate over string
    for c in s.bytes() {
        println!("{}", c);
    }
}
