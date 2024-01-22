#[derive(Debug)] // Manually opt in to print debug info
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };

    // The :? tells the macro we want to use the Debug output form
    println!("rec1 is {:?}", rect1);

    println!(
        "The are of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;

    // The dbg macro returns ownership of rect2
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(rect2);

    let rect3 = Rectangle {
        width: 50,
        height: 100,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect3.area()
    );

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));
    println!("Can rect4 hold rect6? {}", rect4.can_hold(&rect6));

    let rect7 = Rectangle::square(3);

    println!("{:?}", rect7)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}