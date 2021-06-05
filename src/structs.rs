#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    #[allow(dead_code)]
    fn new(width: u32, height: u32) -> Rectangle
    {
        Rectangle
        {
            width,
            height,
        }
    }

    #[allow(dead_code)]
    fn square(size: u32) -> Rectangle
    {
        Rectangle::new(size, size)
    }

    #[allow(dead_code)]
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width >= other.width && self.height >= other.height
    }
}