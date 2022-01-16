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
}
