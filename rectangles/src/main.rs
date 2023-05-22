#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! prints out during run time to the std error stream, includes file and line number
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1); // The above derive debug is required to print this out.

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1),
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
