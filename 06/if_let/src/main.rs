// section 6.3
#[derive(Debug)]
enum UsState {
    Alaska,
    Florida,
    Alabama,
    Arizona,
    Califonia,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // Top code using if-let

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // use if-let when you want to do anything with the other match arms.
    // if-let don't consider all possibilities so it is Not Exhaustive

    //Another match is like this:

    let coin = Coin::Quarter(UsState::Califonia);
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }

    // using if-let-else:
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("non-quarter coin count: {}", count)
}
