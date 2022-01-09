// section 5.3
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        // it is better to define each method in a separate imple
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        // construction, note that there is no "&self"
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };
    let rect2 = Rectangle {
        width: 3,
        height: 4,
    };

    let rect3 = Rectangle {
        width: 12,
        height: 6,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("The width of the rectangle is {} pixels", rect1.width);
    println!("The width is greater than zero? {}", rect1.width());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq1 = Rectangle::square(10);
    println!("{:#?}", sq1);
}
