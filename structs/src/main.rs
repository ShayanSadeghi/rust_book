fn main() {

  struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,  
  }
  
  let mut user1 = User {
    email: String::from("some@mail.com"),
    username: String::from("some_user"),
    active: true,
    sign_in_count:1,
  };
  
  user1.email = String::from("another@mail.com");

  println!("{}", user1.email);
  println!("{}", user1.active);
}


