// section 5.3
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("The width of the rectangle is {} pixels", rect1.width);
    println!("The width is greater than zero? {}", rect1.width());
}
