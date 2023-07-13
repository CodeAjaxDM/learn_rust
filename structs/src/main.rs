
struct Rectangle
{
    height: u32,
    width: u32,
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        self.height * self.width
    }

    fn has_width(&self) -> bool
    {
        self.width > 0
    }

    fn has_height(&self) -> bool
    {
        self.height > 0
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool
    {
        rectangle.width <= self.width && rectangle.height <= self.height
    }
}


fn main() 
{
    let rect_1 = Rectangle
    {
        height: 32,
        width: 10,
    };

    let rect_2 = Rectangle
    {
        height: 10,
        width: 10,
    };

    let rect_3 = Rectangle
    {
        height: 10,
        width: 12,
    };

    println!("The area of the recangle is {} square pixels.",
        rect_1.area());

        println!("Can rect2 fit in rect1? {}", rect_1.can_hold(&rect_2));
        println!("Can rect3 fit in rect1? {}", rect_1.can_hold(&rect_3));
}


