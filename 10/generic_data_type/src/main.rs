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

struct Point_2<T> {
    x: T,
    y: T,
}
impl<T> Point_2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only for Point_2 which has f32 type
impl Point_2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// use different generics in method and impl
impl<T, U> Point<T, U> {
    fn mixup<M, N>(self, other: Point<M, N>) -> Point<T, N> {
        Point {
            x: self.x,  // generic type U
            y: other.y, //generic type N
        }
    }
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

    let p2 = Point_2 { x: 1, y: 4 };
    println!("p.x = {}", p2.x());
}
