#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main_rect() {
    let rect1 = Rectangle { width:30, height: 50};
    println!("The area of {:?} is {}", &rect1, area(&rect1));
    println!("The area of {:?} is {}", &rect1, rect1.area());
}