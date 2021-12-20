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
  

  let user2 = User{
    sign_in_count: 0,
    ..user1 // user1 won't be usable after this, because borrowing email and username
  };

  let user3 = User{
    email: String::from("user3@mail.com"),
    username: String::from("user3"),
    ..user2 // user2 will be usable because email and value entered so we don't move them to user3, other values will be copied
  };
  
  println!("{}", user2.email);

  
  // tuple struct

  // these two struct look similar, but they are actualy unrelated to eachother
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32); 

  let black = Color(0,0,0);
  let origin = Point(0,0,0);




}


