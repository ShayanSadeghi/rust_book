// section 10.1

// "largest" is a "generic" over some type "T".
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
} // there will be an error, which we should learn traits to solve that

// define a struct with two generic type
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![3, 5, 6, 1, 21, 4];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['v', 'e', 'c', 'd', 'a', 'u'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 }; // in this situation we can use only one generic
    let both_integer = Point { x: 5, y: 10.0 }; // in this situation we need both generic types
}
