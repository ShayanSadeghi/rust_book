// section 6.3
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
}
