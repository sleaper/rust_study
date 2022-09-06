use std::io;

fn main() {
    println!("Convert string to pig lating: ");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    word = word.trim().to_string();

    let mut pig_latin = String::new();
    let c = word.chars().nth(0).unwrap();

    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
        pig_latin.push_str(&word[..]);
        pig_latin.push_str("-hay");
    } else {
        pig_latin.push_str(&word[1..]);
        pig_latin.push('-');
        pig_latin.push_str(&word[0..1]);
        pig_latin.push_str("ay");
    }

    println!("Pig Latin: {}", pig_latin);
}
