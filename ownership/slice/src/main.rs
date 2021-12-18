fn main() {
  let s = String::from("hello world");
  let word = first_word(&s);

  println!("first word of our String variable is {}", word);

  let s_literal = "hello world";
  let word = first_word(&s_literal);
  
  println!("first word of our literal variable is {}", word);
}

fn first_word(s: &str) -> &str{
  //return the index of the first word.
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate(){
    if item == b' '{
	
      return &s[..i]; // it is same as &s[0..i]
    }
  
  }
  &s[..]
}
