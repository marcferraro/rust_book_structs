use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is shorthand for self: &Self
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect width: {}\nRect height: {}",
            self.width, self.height)
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // Pretty print Rectangle via derive(Debug)
    println!("rect1 is {:#?}", rect1);
    // Prints to stderr, doesn't need derive(Debug)
    dbg!(&rect1);
    // Able to use println! because of fmt::Dispaly implementaiton
    println!("{}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    if rect1.height() {
        println!("The rectangle has a nonzero height; it is {}", rect1.height);
    }
}