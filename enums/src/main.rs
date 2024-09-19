enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Getting rusty")
    }
}

struct IpAddr{
    kind:IpAddrKind,
    address : String,
}

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}


fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddr{
    //     kind : IpAddrKind::V4,
    //     address : String::from("127.0.0.1")
    // }

    let _localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    let _some_number = Some(5);
    let _some_char = Some('e');

    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = None;

    // let sum = x + y.unwrap_or(); errors

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }


}

fn route(ip_kind: IpAddrKind) {
    let _ = ip_kind;
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         => None
//     }
// }
