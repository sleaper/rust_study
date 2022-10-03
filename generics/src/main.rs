// Different generic types
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Option<T, E> {
    Some(T),
    None(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // We can create Point with different types
    let point = Point { x: 5, y: 10.4 };

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("p.x = {}", point.x());
}

// Generic Data Type
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}
