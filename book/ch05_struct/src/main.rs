
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // my realisation
    fn can_hold_my(&self, rect: &Rectangle) -> bool {
        (self.width >= rect.width) & (self.height >= rect.height)
    }

    //book realisation
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    println!("The area of the rectangle is {} square pixels.", area(30, 50));
    println!("The area of the rectangle is {} square pixels.", area_tuple((30, 50)));
    let rect: Rectangle = Rectangle{
        width: 100,
        height: 200
    };
    println!("Rect: {:?}", rect);
    println!("The area of the rectangle is {} square pixels.", area_rectangle(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold_my(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold_my(&rect3));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
