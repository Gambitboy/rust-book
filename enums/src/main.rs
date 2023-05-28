#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let me = Message::Write(String::from("Hello, world!"));
    me.call();

    let _some_number = Some(5);
    let _some_car = Some('e');
    let _absent_number: Option<i32> = None;

    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alaska);
    println!("The coin in cents is: {}", coin1.value_in_cents());
    println!("The coin in cents is: {}", coin2.value_in_cents());
    println!("The coin in cents is: {}", coin3.value_in_cents());
    println!("The coin in cents is: {}", coin4.value_in_cents());

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 7;

    match dice_roll {
        3 => println!("3!"),
        7 => println!("7!"),
        _ => (),
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("The IP type is: {:?}", ip_kind)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
