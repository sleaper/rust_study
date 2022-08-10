enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    //* Smarter way with use of enums
    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //* Smarter way with use of enums
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
