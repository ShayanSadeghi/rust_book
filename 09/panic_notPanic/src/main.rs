//section 9.3
use std::net::IpAddr;

fn main() {
    // parsing a string to IpAddr, returns a Result that may contain Ok or Err
    let home: IpAddr = "127.0.0.1".parse().unwrap(); // this parse return Ok value
                                                     // let home: IpAddr = "921.0.0.1".parse().unwrap(); // it is not a valid ip and return an Err. here we have panic
    println!("{}", home);
}
