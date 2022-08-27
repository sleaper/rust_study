struct User {
    active: bool,
    username: String,
    email: String,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    let mut user = User {
        active: true,
        username: String::from("joe"),
        email: String::from("pepe@pepa.com"),
    };

    let user2 = User {
        active: true,
        ..user // can spread the user struct, must be last
    };

    user.email = String::from("petrik.spac@gmail.com");

    let black = Color(0, 0, 0);

    println!("Hello, world!");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
    }
}
