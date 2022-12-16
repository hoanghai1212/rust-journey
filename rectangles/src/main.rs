#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // dumb way
    // let w1 = 30;
    // let h1 = 50;

    // tuples way
    // let rect1 = (30, 50);

    // struct way
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    println!("The area pf the rectangle is {} square pixels", area(&rect1));
}

// dumb way
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// tuples way
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct way
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
