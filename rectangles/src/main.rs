#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! prints out during run time to the std error stream, includes file and line number
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 25,
    };

    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1); // The above derive debug is required to print this out.

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    if rect1.can_hold(&rect2) {
        println!("The first rectangle can hold the second");
    }

    let sqr = Rectangle::square(32);
    println!("sqr is {:#?}", sqr);
}
