fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s);
  s.clear(); // we clear s and so we shouldn't have access to word after this line

  // if we use word, which is an immutable variable after mutating the refrence string, s, (using clear) we get an error during the compile time and it is usefull :)

  println!("{}", word);


}

fn first_word(s: &String) -> &str{
  //return the index of the first word.
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate(){
    if item == b' '{
	
      return &s[..i]; // it is same as &s[0..i
    }
  
  }
  &s[..]
}
