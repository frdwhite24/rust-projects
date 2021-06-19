#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is a method because it takes self as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an associated function because they're simply associated
    // with the struct, similar to String::from
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 50,
        height: 25,
    };

    let smaller_rect = Rectangle {
        width: 25,
        height: 10,
    };

    let bigger_rect = Rectangle {
        width: 55,
        height: 30,
    };

    let my_square = Rectangle::square(50);

    println!("Area of my rectangle: {}", my_rectangle.area());
    println!(
        "Can hold smaller rectangle? {}",
        my_rectangle.can_hold(&smaller_rect)
    );
    println!(
        "Can hold bigger rectangle? {}",
        my_rectangle.can_hold(&bigger_rect)
    );
    println!("My square {:#?}", my_square);
}
