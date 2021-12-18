fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s);
  s.clear(); // when we clear the actual string, index stored in "word" variable isn't usefull but we keep it and it is error prone.


}

fn first_word(s: &String) ->usize{
  //return the index of the first word.
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate(){
    if item == b' '{
	
      return i;
    }
  
  }
  s.len()
}
