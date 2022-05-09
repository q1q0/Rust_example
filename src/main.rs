#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.height * self.width
    }
    fn creat_right_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
} 

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    rect1.width = 50;
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.get_area()
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}