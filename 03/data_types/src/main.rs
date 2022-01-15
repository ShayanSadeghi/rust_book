fn main() {
    //=== scalar data types ===
    // integers and unsigned integers (i, u -> 8-128)
    let x: u8 = 255;
    println!("The 8 bit unsigned type max value is: {}", x);

    //float (f32, f64)
    let y = 2.2;
    println!("floating point example {}", y);

    //bolean
    let my_bool_val = true;
    // let my_bool_val2: bool = true; //explicit type annotation
    println!("boolean value {}", my_bool_val);

    //characters
    let c = 'z'; //use single quotation mark instead of double
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Character Example: {}", c);
    println!("Character Example: {}", z);
    println!("Character Example: {}", heart_eyed_cat);

    //=== compound data types ===

    //Tuples
    // Tuples in Rust have fixed length
    let my_tup = ("Shayan", 25, "Engineer");

    //destruction
    let (x, _y, _z) = my_tup; //to avoid a warning, variable names which aren't used in the code should start with an underscore
    println!("My name is {}", x); //warning for defining unused variables (y,z)

    //using tuple index
    let my_new_tup: (String, i8, String) = ("Shayan".to_string(), 25, "Engineer".to_string()); // explicit type annotation
    println!("I am {} years old", my_new_tup.1);

    //Array
    // Arrays in Rust have fixed length
    let a = [1, 2, 3, 4, 5];
    println!("First value in array is {}", a[0]);

    let b: [i32; 5] = [6, 7, 8, 9, 10]; // define the type(i32) and length(5) of the array
    println!("Second value in array b is {}", b[1]);

    let c = [3; 5]; // == [3,3,3,3,3]
    println!("the first value of array c {}", c[0])
}
