struct Rectangle {
    width: u32,
    height: u32,
}

fn cal_rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 50,
    };

    println!("rectangle area: {}", cal_rectangle_area(rectangle))
}