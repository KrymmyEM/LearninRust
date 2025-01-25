

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    println!("The area of the rectangle is {} square pixels.", area(30, 50));
    println!("The area of the rectangle is {} square pixels.", area_tuple((30, 50)));
}
