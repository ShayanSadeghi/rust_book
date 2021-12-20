fn main() {

  let user1 = build_user(String::from("some@mail.com"), String::from("user1234"));
  println!("{}", user1.email);
  println!("{}", user1.active);
}


struct User {
  email: String,
  username: String,
  active: bool,
  sign_in_count: u64,  
}

fn build_user(email: String, username: String) -> User{
  User {
    email, //use shorthand syntax
    username, //use shorthand syntax
    active: true,
    sign_in_count:1,
  }
}
