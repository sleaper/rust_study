const LYRICS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a laying",
    "seven swans a swimming",
    "eight maids a milking",
    "nine ladies dancing",
    "ten lords a leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    for day in 0..12 {
        println!("");

        main_line(day);
        other_text(day);
    }
}

fn main_line(day: i32) {
    println!(
        "On the {} day of Christmas my true love sent to me",
        ORDINALS[day as usize]
    );
}

fn other_text(day: i32) {
    println!("{day}");
    for number in (0..day + 1).rev() {
        println!("{}", LYRICS[number as usize]);
    }
}
