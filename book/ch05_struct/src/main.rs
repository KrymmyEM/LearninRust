struct Rectangle {
    width: u32,
    height: u32,
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
    println!("The area of the rectangle is {} square pixels.", area_rectangle(&rect));

}
