//section 9.3
use std::net::IpAddr;

// define new type
pub struct Guess {
    value: i32,
}
// bound the defined type value
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess { value }
    }
    // getter method
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // parsing a string to IpAddr, returns a Result that may contain Ok or Err
    let home: IpAddr = "127.0.0.1".parse().unwrap(); // this parse return Ok value
                                                     // let home: IpAddr = "921.0.0.1".parse().unwrap(); // it is not a valid ip and return an Err. here we have panic
    println!("{}", home);

    let x = Guess::new(10);
    let x = Guess::new(101);
    println!("x is {}", x.value());
}
