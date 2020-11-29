use crate::enums::Message::Write;

enum IpAddr {
    /*
    V4,
    V6,

    Concise version of struct implementation
    of enum below
    V4(String),
    V6(String),

    Refactored further
    */

    V4(u8,u8,u8,u8),
    V6(String)
}

/*struct IpAddr {
    kind: IpAddrKind,
    address: String,
}*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),

}

impl Message {
    fn call(&self) {
        // method would would be defined here
    }
}

pub(crate) fn main_c() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    /* Concise version below
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };*/

    // Refactored to encoded enum type version
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    // Option examples
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

}

fn route(ip_kind: IpAddr) {}

