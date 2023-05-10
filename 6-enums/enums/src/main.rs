// Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// the name of each enum variant that we define also becomes a function that constructs an instance of the enum
#[derive(Debug)]
enum IpAddrBetter {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message is {:?}", &self)
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Dollar,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match should handle all cases. Can use "other =>"" to handle all the other cases. _ => can be used but the value cannot be used for this case.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 100, // () => do nothing
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // this is Option<i32> since it is declared as Some
    let some_number = Some(5);
    let six = plus_one(some_number);
    let absent_number: Option<i32> = None;

    let home2 = IpAddrBetter::V4(127, 0, 0, 1);
    let home3 = IpAddrBetter::V6(String::from("::1"));
    let coin_quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!(
        "{:?} {} {:?} {:?}, some number: {:?}, six: {:?} , {:?}, {:?}",
        home.kind, loopback.address, home2, home3, some_number,six, absent_number, coin_quarter
    );

    let m = Message::Write(String::from("hello"));
    m.call();
    let config_max = Some(3u8); // 3 as uint8
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // Same as match above BUT you lose the exhaustive checking that match enforces
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
