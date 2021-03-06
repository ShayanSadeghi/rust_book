// section 6.2
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
    Florida,
    Ohio,
    // etc
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //pattern
        Coin::Penny => {
            //code
            println!("It is a Penny!");
            1
        } //comma separating each arm
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Florida));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // _ don't bind value and () do nothing in this situations
    }
}

fn add_fancy_hat() {
    println!("add a fancy hat");
}

fn remove_fancy_hat() {
    println!("remove fancy hat");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("Noting done and reture None");
            None
        }
        Some(i) => {
            println!("Adding one to the value");
            Some(i + 1)
        }
    }
}
