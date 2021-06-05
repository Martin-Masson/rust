#![allow(unused)]

struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn new(width: u32, height: u32) -> Rectangle
    {
        Rectangle
        {
            width,
            height,
        }
    }

    fn square(size: u32) -> Rectangle
    {
        Rectangle::new(size, size)
    }

    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width >= other.width && self.height >= other.height
    }
}