// section 8.1
// Vectors
fn main() {
    let v_1: Vec<i32> = Vec::new();
    println!("{:?}", v_1);

    let v_2 = vec![1.0, 2.2, 3.1]; //use "vec!" can determine type of elements in vector automatically
    println!("{:?}", v_2);
    {
        let mut v_3 = Vec::new();

        v_3.push(1);
        v_3.push(6);
        v_3.push(3);
        // all values pushed to new vector are in a same type, so rust can determine the type
        println!("{:?}", v_3);
    } // v3 is no longer accessible here

    let v_2_1: &f32 = &v_2[1]; // if there is no element with index 1, the program will panic
    println!("{}", v_2_1);

    match v_2.get(1) {
        Some(val) => println!("The second element is {}", val),
        None => println!("There is no value with this index"),
    }

    let mut v_4 = vec![1, 2, 3, 4, 5];
    // the next to lines cause an error together,
    let first = &v_4[0];
    // v_4.push(6);
    println!("The first element is {}", first)
}
