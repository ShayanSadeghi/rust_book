fn main() {
   let x: i8 = 6;

   if x % 4 == 0 {
      println!("Number is divisible by 4");
   } else if x % 3 == 0 {
      println!("Number is divisible by 3");
   } else if x % 2 == 0 {
      println!("Number is divisible by 2");
   } else {
      println!("Number is not divisible by 4,3 and 2");
   }
}
