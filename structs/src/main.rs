
struct Rectangle
{
    height: u32,
    width: u32,
}

fn main() 
{
    let rect_1 = Rectangle
    {
        height: 32,
        width: 10,
    };

    println!("The area of the recangle is {} square pixels.",
        area(&rect_1 ));
}

fn area(rectangle: &Rectangle) -> u32
{
    rectangle.height * rectangle.width
}
