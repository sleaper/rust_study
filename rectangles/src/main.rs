#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width > self.width && other.height > self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30, //dbg!(30 * scale) = debug example
        height: 50,
    };

    let rect2 = Rectangle {
        width: 80,
        height: 100,
    };

    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    print!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//? Read once more /Method syntax/ and /borrowing/
