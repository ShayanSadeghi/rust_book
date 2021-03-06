fn main() {
   // if - else
   let x: i8 = 6;

   // conditions explicitly should be boolean not numbers etc...
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

   // Repetition
   // "loop" run until we break
   let mut count = 0;
   'counting_up: loop {
      // add label for loop
      println!("count = {}", count);
      let mut remaining = 10;
      loop {
         println!("remaining = {}", remaining);
         if remaining == 9 {
            break; //break inner loop
         }
         if count == 2 {
            break 'counting_up; //break outer loop (with label counting_loop)
         }
         remaining -= 1;
      }
      count += 1;
   }
   println!("End count = {}", count);

   let mut counter = 0;
   let result = loop {
      counter += 1;
      if counter == 10 {
         break counter * 2; //returned value from loop
      }
   };
   println!("result is {}", result);

   //while
   let mut x = 3;
   while x != 0 {
      println!("{}", x);
      x -= 1;
   }

   //for
   let arr = [1, 3, 5, 6, 10];
   for a in arr {
      println!("value in array: {}", a);
   }

   for number in (1..4).rev() {
      println!("{}!", number);
   }
   println!("Run!")
}
