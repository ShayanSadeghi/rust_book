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
}
