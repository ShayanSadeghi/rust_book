fn main() {
   // if - else
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

   let condition = true;
   let number = if condition { 5 } else { 2 }; // values in each arm (if-else) should be in a same type
                                               //and blocks should be expression not statement
   println!("The number is {}", number);
}
