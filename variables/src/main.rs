//type of const should be declared
const THREE_HOURS_IN_SECONS: u32 = 3 * 60 * 60;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // if we don't use "mut", we can't change x now
    x = 6;
    println!("The value of x is {}", x);
    println!(
        "The value of THREE_HOURS_IN_SECONS is {}",
        THREE_HOURS_IN_SECONS
    );
    //shadowing:
    let y = 2;
    println!("The value of y is {}", y);
    let y = y + 1;
    println!("The value of y is {}", y);
    {
        let y = y * 2;
        println!("The value of y is {}", y);
    }
    println!("The value of y is {}", y);

    //type changing is not possible for mutable variables, we do this by shadowing
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The spaces value is {}", spaces)
}
